<!-- SPDX-License-Identifier: CC0-1.0 -->

# Simulator Component — Implementation Tasks

## State Struct

- [x] Derive `Copy` and `Clone` on `State` struct
- [x] Implement `Add<Output = State>` for `State` (component-wise
      addition)
- [x] Implement `Mul<f64, Output = State>` for `State` (component-wise
      multiplication by scalar)
- [x] Implement `State::initial()` function returning all four fields as
      zero

## Friction

- [x] Define `RailFriction` trait with `force(&self, cart_velocity: f64,
      normal_force: f64) -> f64` method
- [x] Implement `smoothed_sign` helper function for `CoulombFriction`
      (e.g., `(v / epsilon).tanh()` with small epsilon)
- [x] Define `CoulombFriction` struct with `pub mu: f64` field
- [x] Implement `RailFriction` for `CoulombFriction` using
      `smoothed_sign` to compute friction force
- [x] Define `PivotFriction` trait with `torque(&self,
      angular_velocity: f64) -> f64` method
- [x] Define `ViscousFriction` struct with `pub damping: f64` field
- [x] Implement `PivotFriction` for `ViscousFriction` using
      `-self.damping * angular_velocity` formula

## Simulator

- [x] Define `Simulator<F: RailFriction>` struct with five public
      fields: `cart_mass`, `pole_mass`, `pole_length`, `gravity`,
      `friction`
- [x] Implement `Simulator::advance()` method: apply impulse kick to
      `cart_velocity`, then delegate to `solver::step()`
- [x] Implement `DynamicalSystem` trait for `Simulator<F>` with
      `type State = State`
- [x] Implement `derivative()` method computing cart acceleration via
      equation: `ẍ = (f + m·L·θ̇²·sin(θ) − m·g·sin(θ)·cos(θ)) /
      (M + m·sin²(θ))`
- [x] Implement `derivative()` method computing pole angular
      acceleration via equation: `θ̈ = (g·sin(θ) − ẍ·cos(θ)) / L`
- [x] In `derivative()`, compute normal force as `(self.cart_mass +
      self.pole_mass) * self.gravity` and obtain friction force via
      `self.friction.force()`
- [x] Modify `Simulator<F>` struct to add second generic parameter
      `P: PivotFriction` and new `pivot_friction: P` field
- [x] Update `derivative()` method cart-acceleration formula to
      include `−τ·cos(θ)/L` term in numerator, where
      `τ = self.pivot_friction.torque(state.pole_angular_velocity)`
- [x] Update `derivative()` method pole-angular-acceleration formula
      to include `+ τ/(m·L²)` term, where
      `τ = self.pivot_friction.torque(state.pole_angular_velocity)`
- [x] Update existing test struct literals (`test_gravity_only_sanity_check`,
      `test_friction_damping`) to include `pivot_friction` field
      (plumbing for compilation, e.g. `ViscousFriction { damping: 0.0 }`)

## Verification

- [x] Write unit test: gravity-only sanity check with `mu = 0`, zero
      impulse; assert pole swings and total energy stays roughly bounded
- [x] Write unit test: friction damping check with nonzero `mu` and an
      initial impulse; assert cart velocity trends toward zero over time
- [x] Write unit test: pivot damping check with pole displaced or
      initial angular velocity, nonzero pivot damping, rail friction
      disabled; assert pole angular velocity trends toward zero over time
- [x] Update `App::create()` in `web.rs` to construct `Simulator`
      with `ViscousFriction { damping: <default> }` as second generic
      parameter (hard requirement for `--features web` compilation)
