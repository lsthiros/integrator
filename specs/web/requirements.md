<!-- SPDX-License-Identifier: CC0-1.0 -->

# Web UI Component Requirements

## Overview

The web UI is a single-page application (SPA) that visualizes and allows
user interaction with a cart-and-pole dynamical system. Users view the
simulation in a web browser on a host other than the machine running the
simulator. The UI drives the simulator at each frame/tick, renders the
current system state, and accepts user input to apply impulses to the cart.
The UI is implemented using the Yew framework, a Rust-to-WebAssembly
frontend framework, consistent with this project's decision to keep the
entire simulation-and-UI stack in Rust (see YEW.md for research notes).

## User Stories

- As a user, I can view the cart-and-pole system rendered visually in a render
  window, with the cart on a rail and a pole hinged to the cart.
- As a user, I can see the simulation update smoothly and in real-time as the
  system evolves.
- As a user, I can apply an impulse or "push" to the cart by interacting with
  the application (via button, keystroke, or similar input method).
- As a user, I can adjust simulation parameters (e.g., gravity, cart mass, pole
  mass, pole length) to explore different dynamics.
- As a user, I can understand how to interact with the application through
  discoverable controls.
- As a user, I can view a running log of recent events (impulses applied,
  parameter changes, resets) so I can see a history of my interactions
  with the simulation.

## Inputs and Outputs

**Inputs (User-facing):**
- Impulse/push commands (button click, keyboard press, or other interactive
  control)
- Parameter adjustment inputs (sliders, text fields, or similar) for tunable
  simulation values
- Page navigation/initialization (e.g., browser page load)

**Outputs (User-facing):**
- Real-time visual rendering of the cart-and-pole state (position, angle,
  motion)
- Display of current parameter values
- Smooth, continuous animation at the simulation's tick rate

**Component Boundaries (for reference):**
- Consumes: State data from the simulator component (cart position, pole angle,
  velocities, etc.)
- Does not directly consume: Integrator (an internal detail of the simulator)
- May be served by: Optional HTTP server component (external; not part of this
  component's scope)

## Acceptance Criteria

- The render window displays the cart on a rail and the pole attached to the
  cart, with both visual representations updating as the simulation advances.
- The application updates at the simulation's native tick rate without dropping
  frames or stuttering.
- User controls for applying impulses are clearly visible or easily discoverable
  (e.g., labeled buttons, on-screen instructions, or standard keyboard
  bindings).
- The user can adjust at least the following parameters and observe their effect
  on the simulation:
  - Gravity
  - Cart mass
  - Pole mass
  - Pole length (or similar system parameter)
- Parameter changes take effect on the next simulation update.
- The application remains responsive to user input while the simulation is
  running.
- The page loads and initializes the simulation without errors when accessed
  from a browser.
- A log panel displays a scrolling, transient history of user-triggered
  events (impulses, parameter changes, resets) as they occur, with the
  newest entry visible without additional scrolling.

## Out of Scope

- **Simulator physics and solver logic:** The underlying dynamics, equations of
  motion, and numerical integration are implemented by the simulator component
  and are not part of the web UI.
- **HTTP server implementation:** Serving static files, WASM modules, and
  handling HTTP requests is handled by an optional server component (or build
  toolchain) and is not part of this component.
- **Advanced visualizations or effects:** Particle effects, multiple rendering
  modes, 3D rendering, or other graphics enhancements beyond basic 2D rendering
  of the cart and pole.
- **Persistent state or data logging:** Saving simulation runs to disk,
  replay functionality, or telemetry collection. This does not preclude
  the transient, in-memory event log displayed within the UI itself (see
  Acceptance Criteria), which clears on page reload and is never written
  anywhere.
- **Mobile-specific UI optimization:** While the UI should work on mobile
  browsers, mobile-specific gestures or responsive design optimizations are not
  requirements.
