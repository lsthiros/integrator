<!-- SPDX-License-Identifier: CC0-1.0 -->

# Integrator Component — Requirements

## Overview

The integrator is a general-purpose numerical solver for ordinary
differential equations (ODEs). It implements the Runge-Kutta 4th-order (RK4)
integration method, enabling simulation of continuous dynamical systems by
stepping them forward in time given only knowledge of their instantaneous
derivatives.

The integrator's primary consumer is the simulator component, which will use
it to advance the state of the cart-and-pole system each simulation tick.
The integrator's design must support arbitrary dynamical systems (not just
cart-and-pole) to serve as a reusable learning tool.

## Behavior / Consumer Stories

**As a simulator component:**
- I want to provide my dynamical system to an integrator and ask it to advance time by a given step duration.
- I want to supply the integrator with a system that can compute its own derivatives at any given state, so the integrator can sample those derivatives at strategic points during a step.
- I want the integrator to return the new state after one step, accurately tracking how my system evolved.

**As a learning tool:**
- The integrator should be general enough that other code (e.g., a test Polynomial system) could use it without modification, demonstrating that it is not hard-wired to cart-and-pole physics.

## Inputs / Outputs

**Conceptual Input:**
- A dynamical system object capable of computing its own derivatives given a state and time
- An initial state (position, velocity, or analogous variables)
- A time step duration

**Conceptual Output:**
- The evolved state after one integration step

**System Abstraction:**
- The integrator does not know or care about the structure of the state (e.g., whether it's a scalar, a 2D position, or a 4-vector of cart position, velocity, pole angle, and angular velocity). It operates on an opaque representation.
- The system object is responsible for supplying derivatives when the integrator requests them; the integrator queries the system, not the reverse.

## Acceptance Criteria

1. **Solver Correctness:** The integrator produces results accurate to 4th-order for a known analytic solution (e.g., a polynomial or exponential system) over multiple steps.
2. **Abstraction:** The integrator's public interface accepts a generic system representation; a test system (e.g., Polynomial) can be instantiated and solved without modifying the integrator itself.
3. **Simulator Compatibility:** The simulator component can instantiate the integrator, provide its cart-and-pole dynamics as a system object, and call the integrator to step forward one tick.
4. **Step Size Control:** The integrator accepts arbitrary (positive) step sizes and produces stable, physically reasonable results even as step size varies.
5. **No Hard-Coded Physics:** The integrator contains no knowledge of carts, poles, gravity, or any domain-specific constants; all physics is supplied by the system object.

## Out of Scope

- **Concrete dynamical system implementations** (e.g., Polynomial, cart-and-pole physics): These belong in the simulator and test modules, not in the integrator itself.
- **Adaptive step size or error control:** The integrator accepts a fixed step size per call; adaptive strategies are not part of this component.
- **Web UI or rendering:** No visualization code belongs here.
- **Initial condition selection or parameter tuning:** The integrator does not prescribe how initial states or system parameters are chosen.

