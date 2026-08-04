<!-- SPDX-License-Identifier: CC0-1.0 -->

# Integrator Component — Implementation Tasks

## Module Setup

- [x] Create `integrator/src/solver.rs` as the solver module
- [x] Expose `solver` module publicly from `integrator/src/lib.rs`

## Core Trait

- [x] Define `DynamicalSystem` trait with `State` associated type bounded
      by `Copy + Add<Output = Self::State> + Mul<f64, Output =
      Self::State>`
- [x] Add `derivative(&self, state: Self::State) -> Self::State` method
      to `DynamicalSystem` trait

## RK4 Step Function

- [x] Implement `step<S: DynamicalSystem>(system: &S, state: S::State,
      dt: f64) -> S::State` function
- [x] Compute k1 (derivative at current state)
- [x] Compute k2 (derivative at midpoint estimate using k1)
- [x] Compute k3 (derivative at midpoint estimate using k2)
- [x] Compute k4 (derivative at endpoint estimate using k3)
- [x] Return `state + (k1 + 2*k2 + 2*k3 + k4) * (dt / 6.0)`

## Test System and Verification

- [x] Define `Decay` struct with scalar `f64` state
- [x] Implement `DynamicalSystem` for `Decay` with `derivative(y)`
      returning `-y`
- [x] Derive `Copy` for `Decay`
- [x] Write unit test `test_rk4_convergence_order` that integrates
      `Decay` at step size `h` and `h/2` over a fixed total time interval
- [x] Compute error at `h` by comparing final result to closed-form
      `y0 * e^(-t_total)`
- [x] Compute error at `h/2` the same way
- [x] Assert that the `h/2` error is approximately `1/16` the `h` error
      (within a factor of 2 either side of 16×, to account for
      higher-order terms), demonstrating 4th-order convergence
