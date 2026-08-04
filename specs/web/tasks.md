<!-- SPDX-License-Identifier: CC0-1.0 -->

# Web Component — Implementation Tasks

## Module Setup and Cargo Feature

- [x] Add `yew` dependency to `Cargo.toml` with features `["csr"]` and
      optional flag
- [x] Add `gloo-timers` dependency to `Cargo.toml` with optional flag
- [x] Define `web` feature in `Cargo.toml` that depends on `dep:yew` and
      `dep:gloo-timers`
- [x] Create `integrator/src/web.rs` module file
- [x] Add `#[cfg(feature = "web")] pub mod web;` to
      `integrator/src/lib.rs`

## Build Tooling and xtask

- [x] Create `integrator/index.html` static file with single root `<div
      id="app">` for Yew mount point and link to `styles.css` with
      `<link data-trunk rel="css" href="styles.css">`
- [x] Create `integrator/styles.css` static file with CSS Grid layout
      rules
- [ ] Modify `xtask` to add `web` subcommand that shells out to
      `trunk build` with working directory `integrator/`
- [ ] Modify `xtask` to add `web serve` subcommand that shells out to
      `trunk serve`

## Layout and Styling

- [x] Define `.app` CSS rule with `display: grid`,
      `grid-template-columns: 260px 1fr`, and `height: 100vh`
- [x] Define `.right` CSS rule as nested grid with
      `grid-template-rows: 1fr 200px`
- [x] Style `.controls` column with appropriate padding, background, and
      scrolling if needed
- [x] Style `.viewer` container for SVG rendering with sensible default
      dimensions
- [x] Style `.log` container with `overflow-y: auto`, `height: 200px`,
      and readable font sizing

## Component Structure

- [x] Define `pub struct App` with fields: `simulator:
      Simulator<CoulombFriction>`, `state: State`, `log:
      VecDeque<String>`, `_ticker: gloo_timers::callback::Interval`
- [x] Implement `Component` trait on `App` struct
- [x] Define `pub enum Msg` with variants: `Tick`, `Push(f64)`,
      `SetGravity(f64)`, `SetCartMass(f64)`, `SetPoleMass(f64)`,
      `SetPoleLength(f64)`, `SetFriction(f64)`, `Reset`
- [x] Define `TICK_MS` constant for tick interval (e.g., 16 milliseconds)
- [x] Define `DT` constant as `TICK_MS as f64 / 1000.0` for time step
- [x] Define `IMPULSE_MAGNITUDE` constant for push button force
- [x] Define `MAX_LOG_ENTRIES` constant as 50
- [x] Define `PIXELS_PER_METER` constant for coordinate scaling
- [x] Define `ORIGIN_PX_X` and `ORIGIN_PX_Y` constants for SVG coordinate
      origin

## Initialization

- [x] Implement `create()` method that instantiates default
      `Simulator<CoulombFriction>` with initial parameters
- [x] In `create()`, set `state = State::initial()`
- [x] In `create()`, initialize `log` as empty `VecDeque` with capacity
      `MAX_LOG_ENTRIES`
- [x] In `create()`, create `_ticker` by calling
      `gloo_timers::callback::Interval::new(TICK_MS, move || {
      link.send_message(Msg::Tick) })`
- [x] Verify that dropping `App` automatically drops `_ticker` and
      cancels the interval

## Message Handling

- [x] Implement `update()` method accepting `&mut self`, `msg: Msg`, and
      `ctx: &Context<Self>`, returning `bool`
- [x] In `Msg::Tick` arm, call `self.simulator.advance(self.state, 0.0,
      DT)` and update `self.state`
- [x] In `Msg::Push(mag)` arm, call `self.simulator.advance(self.state,
      mag, DT)` and update `self.state`
- [x] In `Msg::Push(mag)` arm, append formatted log entry describing push
      magnitude and direction
- [x] In `Msg::SetGravity(val)` arm, mutate `self.simulator.gravity` and
      append log entry
- [x] In `Msg::SetCartMass(val)` arm, mutate `self.simulator.cart_mass`
      and append log entry
- [x] In `Msg::SetPoleMass(val)` arm, mutate `self.simulator.pole_mass`
      and append log entry
- [x] In `Msg::SetPoleLength(val)` arm, mutate
      `self.simulator.pole_length` and append log entry
- [x] In `Msg::SetFriction(val)` arm, mutate
      `self.simulator.friction.mu` and append log entry
- [x] In `Msg::Reset` arm, reset `self.state = State::initial()` and
      append log entry
- [x] Ensure all `update()` arms return `true` to trigger re-render

## Rendering: Main View

- [x] Implement `view()` method returning `Html` wrapped in
      `<div class="app">`
- [x] In `view()`, create `<div class="controls">` container calling
      `view_controls()`
- [x] In `view()`, create `<div class="right">` container holding
      `view_viewer()` and `view_log()` outputs
- [x] Return complete `html!` tree from `view()`

## Rendering: Viewer

- [x] Implement `view_viewer()` method returning `Html` for SVG canvas
      wrapped in `<div class="viewer">`
- [x] Render rail as `<line>` element spanning viewer width at
      `ORIGIN_PX_Y` with visible stroke
- [x] Render cart as `<rect>` element centered at `(x, y)` where `x =
      ORIGIN_PX_X + state.cart_position * PIXELS_PER_METER` and
      `y = ORIGIN_PX_Y`
- [x] Compute pole tip coordinates: `tip_x = cart_x +
      simulator.pole_length * PIXELS_PER_METER *
      state.pole_angle.sin()` and `tip_y = ORIGIN_PX_Y -
      simulator.pole_length * PIXELS_PER_METER *
      state.pole_angle.cos()`
- [x] Render pole as `<line>` element from cart center to pole tip with
      visible stroke

## Rendering: Controls

- [x] Implement `view_controls()` method returning `Html` for control
      panel wrapped in `<div class="controls">`
- [x] Add "Push Left" button dispatching `Msg::Push(-IMPULSE_MAGNITUDE)`
- [x] Add "Push Right" button dispatching `Msg::Push(IMPULSE_MAGNITUDE)`
- [x] Create gravity range slider with `min="0"`, `max="20"`, `value =
      simulator.gravity.to_string()`
- [x] Add `oninput` handler to gravity slider parsing value as `f64` and
      dispatching `Msg::SetGravity(val)`
- [x] Add numeric readout displaying gravity with unit label "m/s²"
- [x] Create cart mass range slider with `min="0.1"`, `max="10"`,
      `value = simulator.cart_mass.to_string()`
- [x] Add `oninput` handler to cart mass slider dispatching
      `Msg::SetCartMass(val)` with numeric readout and label "kg"
- [x] Create pole mass range slider with `min="0.1"`, `max="10"`,
      `value = simulator.pole_mass.to_string()`
- [x] Add `oninput` handler to pole mass slider dispatching
      `Msg::SetPoleMass(val)` with numeric readout and label "kg"
- [x] Create pole length range slider with `min="0.1"`, `max="5"`,
      `value = simulator.pole_length.to_string()`
- [x] Add `oninput` handler to pole length slider dispatching
      `Msg::SetPoleLength(val)` with numeric readout and label "m"
- [x] Create friction coefficient range slider with `min="0"`,
      `max="2"`, `value = simulator.friction.mu.to_string()`
- [x] Add `oninput` handler to friction slider dispatching
      `Msg::SetFriction(val)` with numeric readout and label "μ"
- [x] Add "Reset" button dispatching `Msg::Reset` on click

## Event Log

- [x] Implement `view_log()` method returning `Html` for event log
      wrapped in `<div class="log">`
- [x] Render log entries as individual `<div>` elements, oldest first
      (top) to newest (bottom)
- [x] Ensure newest entry is visible at bottom without user scrolling by
      default
- [x] In `update()`, implement helper `push_log(entry: String)` that
      appends entry to `self.log`
- [x] In `push_log()`, if `self.log.len() >= MAX_LOG_ENTRIES`, pop oldest
      entry from front before pushing new entry

## Verification

- [x] Run `cargo xtask web serve` and confirm server starts without errors
- [ ] Open browser to served page and confirm it loads without errors
- [ ] Verify cart-on-rail and pole rendering appear and are visible
- [ ] Click "Push Left" button, observe cart accelerates leftward
- [ ] Click "Push Right" button, observe cart accelerates rightward
- [ ] Adjust gravity slider, observe pole swing behavior changes
- [ ] Adjust cart mass slider, observe dynamics respond appropriately
- [ ] Adjust pole mass slider, observe dynamics respond appropriately
- [ ] Adjust pole length slider, observe pole visual and dynamics change
- [ ] Adjust friction slider, observe cart velocity damping changes
- [ ] Click "Reset" button, verify cart and pole return to initial state
- [ ] Perform action (push or parameter change), verify log entry appears
- [ ] Perform multiple actions, verify newest log entry appears at bottom
- [ ] Perform >50 actions, verify oldest entries are removed and log never
      exceeds `MAX_LOG_ENTRIES`
- [ ] Confirm simulation animates smoothly without stuttering or frame
      drops
- [ ] Verify all acceptance criteria from requirements.md are satisfied
