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
`m` and length `L` pivoting on the cart, under gravity `g` (acting in `-y`), a
friction force `f` acting on the cart along the rail, and a friction torque `τ`
acting at the pivot (opposing the pole's rotation):

```text
ẍ = (f + m·L·θ̇²·sin(θ) − m·g·sin(θ)·cos(θ) − τ·cos(θ)/L) / (M + m·sin²(θ))
θ̈ = (g·sin(θ) − ẍ·cos(θ)) / L + τ/(m·L²)
```

These reduce to the original (pre-pivot-friction) equations when `τ = 0`; `τ`
was re-derived through the same Lagrangian procedure as the rest of this
section, adding `τ` as a generalized non-conservative force on the `θ`
coordinate, rather than bolted on separately.

`f` and `τ` come from the pluggable friction models below. There is no
separate "applied force" term in these equations — the impulse is a discrete
velocity change applied outside of them, per the Modeling Assumptions above.

## Pluggable Rail Friction

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

## Pluggable Pivot Friction

A second, independent trait for friction at the pole's pivot:

```rust
pub trait PivotFriction {
    /// Torque (N·m) opposing the pole's rotation, given its current
    /// angular velocity.
    fn torque(&self, angular_velocity: f64) -> f64;
}
```

Unlike `RailFriction`, this trait takes no normal-force parameter: a hinge has
no equivalent load-bearing quantity the way the rail has cart+pole weight, so
there is nothing physically meaningful to scale a Coulomb-style term by here.
`Simulator` is generic over a second parameter, `P: PivotFriction`, on the same
static-dispatch basis as `RailFriction`.

The required default (and, for now, only) implementation is viscous
(damped-bearing) friction, chosen over Coulomb specifically because it needs
no invented constant and introduces no `sign()` discontinuity at
`angular_velocity = 0`, unlike a Coulomb pivot torque would:

```rust
pub struct ViscousFriction {
    pub damping: f64,
}

impl PivotFriction for ViscousFriction {
    fn torque(&self, angular_velocity: f64) -> f64 {
        -self.damping * angular_velocity
    }
}
```

## Simulator Struct and API

```rust
pub struct Simulator<F: RailFriction, P: PivotFriction> {
    pub cart_mass: f64,
    pub pole_mass: f64,
    pub pole_length: f64,
    pub gravity: f64,
    pub friction: F,
    pub pivot_friction: P,
}

impl<F: RailFriction, P: PivotFriction> Simulator<F, P> {
    pub fn advance(&self, state: State, impulse: f64, dt: f64) -> State {
        let mut kicked = state;
        kicked.cart_velocity += impulse / self.cart_mass;
        solver::step(self, kicked, dt)
    }
}

impl<F: RailFriction, P: PivotFriction> DynamicalSystem for Simulator<F, P> {
    type State = State;
    fn derivative(&self, state: State) -> State {
        // Implements the equations of motion above, with
        // f = self.friction.force(state.cart_velocity, (self.cart_mass + self.pole_mass) * self.gravity)
        // τ = self.pivot_friction.torque(state.pole_angular_velocity)
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

**Pivot damping check**: with the pole displaced from vertical (or given
an initial angular velocity) and nonzero pivot `damping`, and rail
friction disabled, pole angular velocity should trend toward zero over
time, matching the "pivot friction opposes pole rotation" acceptance
criterion.
