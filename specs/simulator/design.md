<!-- SPDX-License-Identifier: CC0-1.0 -->

# Simulator Component — Design

## Placement

The simulator is the `simulator` module of the unified `integrator` crate:
`integrator/src/simulator.rs`, exposed as `integrator::simulator`. It depends on
`integrator::solver::{DynamicalSystem, step}`.

## Modeling Assumptions

- The pole is modeled as a point mass `m` at the end of a massless rigid
  rod of length `L`, pivoting at the cart — a simple pendulum on a cart, not
  a uniform rod with distributed mass and rotational inertia. This deliberate
  simplification means requirements.md's "pole length" and "pole mass" map
  directly onto `L` and `m` under this model. A uniform-rod model would add a
  rotational-inertia term but was not required.

- The rail's normal force is approximated as static: `N = (M + m) * g`, the
  combined weight of cart and pole. The small dynamic fluctuation in vertical
  reaction force caused by the pole's vertical acceleration is not accounted
  for.

- The user-provided "impulse" is treated as an instantaneous change in the
  cart's momentum—a true impulse, not a sustained force. It is applied directly
  to `cart_velocity` before integrating a tick, rather than injected as a force
  term into the continuous equations of motion.

## State Representation

```rust
#[derive(Copy, Clone)]
pub struct State {
    pub cart_position: f64,          // x, meters
    pub cart_velocity: f64,          // ẋ, m/s
    pub pole_angle: f64,             // θ, radians; 0 = upright, measured from +y
    pub pole_angular_velocity: f64,  // θ̇, rad/s
}
```

Implements `Add<Output = State>` and `Mul<f64, Output = State>` component-wise,
satisfying `DynamicalSystem::State`'s trait bound from the solver module.

## Equations of Motion

Derived via Lagrangian mechanics for a cart of mass `M`, a pendulum-pole of mass
`m` and length `L` pivoting on the cart, under gravity `g` (acting in `-y`) and
a friction force `f` acting on the cart along the rail:

```text
ẍ = (f + m·L·θ̇²·sin(θ) − m·g·sin(θ)·cos(θ)) / (M + m·sin²(θ))
θ̈ = (g·sin(θ) − ẍ·cos(θ)) / L
```

`f` comes from the pluggable friction model below. There is no separate "applied
force" term in these equations — the impulse is a discrete velocity change
applied outside of them, per the Modeling Assumptions above.

## Pluggable Friction

```rust
pub trait RailFriction {
    /// Force (N) opposing the cart's motion, given its current velocity
    /// and the normal force the rail is currently supporting.
    fn force(&self, cart_velocity: f64, normal_force: f64) -> f64;
}
```

`Simulator` is generic over `F: RailFriction` (static dispatch, matching the
solver module's `<S: DynamicalSystem>` style rather than `dyn` trait objects),
so friction models are swappable at compile time without touching the equations
of motion above.

The required default implementation is Coulomb (dry sliding) friction:

```rust
pub struct CoulombFriction {
    pub mu: f64,
}

impl RailFriction for CoulombFriction {
    fn force(&self, cart_velocity: f64, normal_force: f64) -> f64 {
        -self.mu * normal_force * smoothed_sign(cart_velocity)
    }
}
```

`smoothed_sign` (e.g. `(v / epsilon).tanh()` for a small `epsilon`) replaces a
hard `sign(v)` so friction direction does not discontinuously flip and chatter
at `v = 0` under RK4's fixed-step evaluation. This smoothing is
`CoulombFriction`'s own responsibility, not the `RailFriction` trait's — a
different implementation is free to handle the zero-velocity case differently.

## Simulator Struct and API

```rust
pub struct Simulator<F: RailFriction> {
    pub cart_mass: f64,
    pub pole_mass: f64,
    pub pole_length: f64,
    pub gravity: f64,
    pub friction: F,
}

impl<F: RailFriction> Simulator<F> {
    pub fn advance(&self, state: State, impulse: f64, dt: f64) -> State {
        let mut kicked = state;
        kicked.cart_velocity += impulse / self.cart_mass;
        solver::step(self, kicked, dt)
    }
}

impl<F: RailFriction> DynamicalSystem for Simulator<F> {
    type State = State;
    fn derivative(&self, state: State) -> State {
        // Implements the equations of motion above, with
        // f = self.friction.force(state.cart_velocity, (self.cart_mass + self.pole_mass) * self.gravity)
    }
}
```

`advance` is what the web UI calls each tick, per requirements.md's "advance the
simulation by one tick with optional impulse" story: it applies the impulse as
an instantaneous velocity kick, then delegates one RK4 step to the solver
module.

Reset (per requirements.md's "reset" story) is a plain associated function:
`State::initial()`, returning the cart at the origin with the pole upright, both
at rest — all four fields zero.

## Error Handling

Same posture as the solver module: infallible, pure functions. `cart_mass > 0`,
`pole_length > 0`, `dt > 0`, and similar preconditions are caller invariants —
the web UI is responsible for only offering valid tunable ranges — and are not
validated at runtime.

## Verification Approach

**Gravity-only sanity check** (`mu = 0`, no impulse): the pole should swing
under gravity, and total mechanical energy should stay roughly bounded (RK4 is
not exactly energy-conserving, but should not grow unboundedly), matching
requirements.md's "reasonable energy behavior" criterion.

**Friction damping check**: with an impulse applied and nonzero `mu`, cart
velocity should trend toward zero over time rather than sustaining indefinitely,
matching the "rail friction opposes cart motion" acceptance criterion.
