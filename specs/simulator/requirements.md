<!-- SPDX-License-Identifier: CC0-1.0 -->

# Simulator Component Requirements

## Overview

The simulator component provides a physics simulation of a cart-and-pole
system (a cart constrained to move along a horizontal rail, with a pole
hinged to the cart). The simulator advances the system's state over time
using the integrator component's RK4 solver, applies gravity, and responds to
user-provided impulse forces. It is the bridge between the integrator (which
provides generic numerical solvers) and the web UI (which renders the system
and collects user input).

## Physical Setup / Conventions

- **Coordinate frame**: The `+y` direction is "up" (opposite the direction of
  gravity); gravity acts in the `−y` direction.
- **Rail and cart**: The rail is a straight line along the `x`-axis,
  perpendicular to gravity; the cart is constrained to slide along it.
- **Physical units**: All physical quantities (masses, lengths, forces, time,
  angles) are expressed in SI units — kilograms, meters, seconds, radians,
  Newtons.

## Consumer Stories

As the **web UI**, I want to:

- **Advance the simulation by one tick** with optional user-provided impulse (or
  zero impulse for free-fall behavior), and receive back the updated state of
  the cart and pole.
- **Query the current state** of the cart and pole at any time without advancing
  the simulation, so I can render it.
- **Reset the system** to an initial or canonical state, so I can restart the
  simulation without reloading the application.
- **Tune system parameters** (e.g., pole length, masses, rail friction,
  pivot friction, gravity magnitude) and have those changes take effect
  on the next tick, so I can experiment with different physical
  configurations.

## Inputs and Outputs

### Inputs

- **Impulse magnitude**: A scalar force magnitude applied to the cart along the
  rail direction (can be zero for unforced dynamics).
- **Time step**: The duration over which to advance the simulation in one tick
  (provided implicitly by the integrator's RK4 step size).
- **System parameters** (tunable):
  - Cart mass (kg)
  - Pole mass (kg)
  - Pole length (m)
  - Rail sliding-friction coefficient (dimensionless)
  - Pivot (rotational) friction coefficient
  - Gravity magnitude (m/s²)

### Outputs

- **Cart position**: The cart's location along the rail (scalar, e.g., meters
  from a reference point).
- **Cart velocity**: The cart's speed along the rail (scalar, can be negative).
- **Pole angle**: The angle of the pole relative to vertical (scalar, e.g.,
  radians, zero = pole upright).
- **Pole angular velocity**: The pole's rate of rotation (scalar, radians per
  unit time).

## Acceptance Criteria

- **Gravity affects the pole** even when no impulse is applied; the pole should
  swing downward under gravity alone.
- **Impulse force moves the cart** in the direction of the applied force; the
  impulse should also indirectly affect the pole's motion through the cart-pole
  coupling.
- **State evolves continuously and smoothly** over successive ticks; the
  integrator's RK4 method should produce physically plausible trajectories.
- **Energy behavior is reasonable** for the given step size; while energy
  conservation is not exact (RK4 is an approximation), energy should not grow
  unboundedly or exhibit pathological oscillations over realistic simulation
  lengths.
- **Parameter changes** (e.g., increasing gravity magnitude, changing pole
  length) produce qualitatively correct changes in system behavior (e.g.,
  stronger gravity causes faster pole swing).
- **The system can be reset** to a well-defined initial state (e.g., cart at
  origin with pole vertical, both at rest).
- **Rail friction opposes cart motion**: Rail friction opposes the cart's motion
  along the rail (sliding/kinetic friction). Cart motion introduced by an
  impulse is damped over time by this friction, in addition to any energy
  exchanged with the pole.
- **Pivot friction opposes pole rotation**: Friction at the pole's pivot
  opposes its angular motion (rotational/viscous friction, proportional
  to angular velocity). Pole rotation introduced by gravity or an
  impulse is damped over time by this friction, independent of rail
  friction.
- **All quantities use SI units**: All configurable constants and reported
  outputs use SI units.

## Out of Scope

- **Integrator internals**: The simulator assumes the integrator correctly
  implements RK4. Testing or modifying the integrator's algorithm is the
  integrator component's responsibility.
- **Rendering and display**: How the cart and pole are drawn on screen, colors,
  animations, and UI layout are the web UI's responsibility. The simulator
  provides only numeric state.
- **User input capture**: Collecting impulse commands from mouse, keyboard, or
  touch input is the web UI's responsibility; the simulator only receives the
  computed impulse magnitude.
- **Persistent storage**: Saving or loading simulation states is out of scope.
- **Advanced physics features**: Constraints beyond the rail, collisions,
  deformable poles, aerodynamic drag, or other real-world complications are not
  required.
