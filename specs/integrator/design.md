<!-- SPDX-License-Identifier: CC0-1.0 -->

# Integrator Component — Design

## Placement

The integrator is not its own crate. Per the Rustonomicon appendix's crate decision, it is the `solver` module of the unified `integrator` crate: `integrator/src/solver.rs`, exposed as `integrator::solver`.

## Core Abstraction

A Rust trait defines the solver's contract with systems:

```rust
pub trait DynamicalSystem {
    type State: Copy + Add<Output = Self::State> + Mul<f64, Output = Self::State>;

    fn derivative(&self, state: Self::State) -> Self::State;
}
```

**State** is opaque to the solver. It may be a scalar (for a test `Decay`
system) or a small fixed-size struct or array (for the simulator's
`[position, velocity, angle, angular_velocity]`). The solver only requires
that states be addable and scalable by `f64`, which is what RK4's weighted
combination requires.

**Derivative** takes a state and returns `d(state)/dt`. For a second-order
physical system reduced to first-order form, the returned state's "position"
slots are the input's "velocity" slots, and its "velocity" slots are
accelerations computed from the system's physics. The system, not the solver,
is responsible for that reduction.

**No time parameter.** Nothing in this project's scope (RK4 for an autonomous cart-and-pole system) requires explicitly time-varying dynamics. If a future system needs it, add it then.

## Step Function

```rust
pub fn step<S: DynamicalSystem>(system: &S, state: S::State, dt: f64) -> S::State
```

Classic RK4: evaluates `derivative` four times (k1 at `state`, k2 and k3 at
successive midpoint estimates, k4 at the endpoint estimate) and returns
`state + (k1 + 2*k2 + 2*k3 + k4) * (dt / 6)`. `dt` is a fixed step per
call—no adaptive step-size control, per requirements.

## Numeric Type

`f64` throughout: states, derivatives, and `dt`. No generic float parameter; this project does not need `f32`, and a generic parameter adds complexity requirements do not ask for.

## Test System (Decay)

The `Decay` system used to verify solver accuracy is a simple autonomous exponential decay system: `dy/dt = -y` with scalar `f64` state. Its closed-form solution is `y(t) = y0 * e^(-t)`. The system lives in `solver.rs`'s `#[cfg(test)]` module (or the crate's `tests/` directory), not as part of the public API. Per requirements, concrete systems are out of scope for this component's shipped surface.

## Accuracy Verification

A unit test verifies RK4's 4th-order convergence by checking convergence order rather than exact-value comparison. The test integrates `Decay` from a known `y0` over a fixed total time interval at step size `h` and again at step size `h/2`. Each run's final result is compared to the closed-form value `y0 * e^(-t_total)` to compute an error at both step sizes. The test asserts that the `h/2` error is approximately `1/16` the `h` error (within a factor of 2 either side of 16×)—this directly demonstrates 4th-order global error (`O(h^4)`) and satisfies the "Solver Correctness" acceptance criterion.

## Error Handling

Both `step` and `derivative` are infallible, pure numeric functions. `dt > 0` is a caller invariant (the simulator's responsibility), not validated at runtime—there is no untrusted boundary here to guard.
