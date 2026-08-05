<!-- SPDX-License-Identifier: CC0-1.0 -->

# Web UI Component — Design

## Placement

The web UI is not its own crate. Per the Rustonomicon appendix, it is the
`web` module of the unified `integrator` crate: `integrator/src/web.rs`,
exposed conditionally as `integrator::web`. It depends on
`integrator::simulator::{Simulator, State, CoulombFriction, ViscousFriction}`.

The `web` module and its dependencies (`yew`, `gloo-timers`) sit behind a
new `web` Cargo feature, so a plain native `cargo build`/`cargo test` on
the `integrator` crate never pulls in wasm-bindgen machinery:

```toml
[dependencies]
yew = { version = "...", features = ["csr"], optional = true }
gloo-timers = { version = "...", optional = true }

[features]
web = ["dep:yew", "dep:gloo-timers"]
```

```rust
#[cfg(feature = "web")]
pub mod web;
```

## Build Tooling: Trunk and xtask

Trunk remains the tool that actually builds and bundles the WASM
artifact: it invokes `cargo build --target wasm32-unknown-unknown
--features web`, runs `wasm-bindgen`, and assembles `index.html` plus a
linked `styles.css` (both new static files living alongside
`integrator/Cargo.toml`, per Trunk's convention).

`xtask` (see the Rustonomicon appendix) gains a `web` subcommand
(`cargo xtask web build` / `cargo xtask web serve`) that shells out to
`trunk build` / `trunk serve` with the correct working directory. This
keeps one command surface for contributors without reimplementing what
Trunk already does well — orchestration, not replacement, matching the
project's existing `xtask` philosophy.

## Layout

The page is a single CSS Grid, two columns: a fixed-width `controls`
column spanning the full height on the left, and a right column split
into two rows — `viewer` on top, `log` beneath it:

```text
+----------+-----------+
| controls |    viewer |
|          +-----------+
|          |    log    |
+----------+-----------+
```

```css
.app {
  display: grid;
  grid-template-columns: 260px 1fr;
  grid-template-rows: 1fr;
  height: 100vh;
}
.right {
  display: grid;
  grid-template-rows: 1fr 200px;
}
```

This CSS lives in a plain `styles.css` linked from `index.html` via
Trunk's `<link data-trunk rel="css" href="styles.css">`, keeping styling
out of Rust string literals.

## Component Structure

A single Yew struct component, `App`, owns all UI state — this is a
small, single-page tool, so splitting `controls`/`viewer`/`log` into
separate child components would add message-passing and prop-drilling
overhead without a corresponding benefit. `view()` renders the three
regions through plain helper methods (`view_controls`, `view_viewer`,
`view_log`) instead.

```rust
pub struct App {
    simulator: Simulator<CoulombFriction, ViscousFriction>,
    state: State,
    log: VecDeque<String>,
    _ticker: gloo_timers::callback::Interval,
}

pub enum Msg {
    Tick,
    Push(f64),
    SetGravity(f64),
    SetCartMass(f64),
    SetPoleMass(f64),
    SetPoleLength(f64),
    SetFriction(f64),
    SetPivotDamping(f64),
    Reset,
}
```

`create()` builds a default `Simulator<CoulombFriction, ViscousFriction>`,
sets `state = State::initial()`, and starts `_ticker` as a
`gloo_timers::callback::Interval` that fires every `TICK_MS`
milliseconds, each tick sending `Msg::Tick` into the component:

```rust
let link = ctx.link().clone();
let ticker = Interval::new(TICK_MS, move || link.send_message(Msg::Tick));
```

Dropping `App` drops `_ticker`, which cancels the interval — there is no
separate teardown step to remember.

## Tick and Impulse Handling

`Msg::Tick` advances the simulation with no impulse:
`self.state = self.simulator.advance(self.state, 0.0, DT)`, where
`DT = TICK_MS as f64 / 1000.0`.

`Msg::Push(magnitude)` is handled the same way but with a nonzero
impulse, and fires immediately from a button click rather than waiting
for the next scheduled tick — so a push feels responsive rather than
delayed by up to one tick interval. It also appends an entry to `log`.

`Msg::SetX(value)` mutates the corresponding public field on
`self.simulator` directly (all six fields are `pub`, per
`specs/simulator/design.md`) and appends a log entry; `Msg::Reset` resets
`self.state` to `State::initial()` and logs the reset. Every arm returns
`true` from `update()` to trigger a re-render.

## Rendering

The viewer renders as inline SVG through the `html!` macro — no
`<canvas>`, no JavaScript interop. A fixed `PIXELS_PER_METER` constant
converts physical units to screen coordinates:

- **Rail**: a static `<line>` spanning the viewer's width.
- **Cart**: a `<rect>` centered at
  `x = origin_px + state.cart_position * PIXELS_PER_METER`.
- **Pole**: a `<line>` from the cart's center to a tip computed from
  `state.pole_angle` and `pole_length * PIXELS_PER_METER`.

Rendering is a pure function of `self.state` and `self.simulator`'s
current parameters — there is no separate rendering state to keep in
sync.

## Controls

The `controls` column holds, per requirements.md's acceptance criteria:

- Two buttons, "Push Left" / "Push Right", dispatching
  `Msg::Push(-IMPULSE_MAGNITUDE)` / `Msg::Push(IMPULSE_MAGNITUDE)`.
- Six labeled `<input type="range">` sliders — gravity, cart mass, pole
  mass, pole length, rail friction coefficient, pivot damping
  coefficient — each paired with a numeric readout of its current
  value, satisfying requirements.md's "display of current parameter
  values" output. Each slider's `oninput` callback parses the new value
  as `f64` and dispatches the matching `Msg::SetX`.
- A "Reset" button dispatching `Msg::Reset`.
- A static copyright/credit line, "© 2026 Louie Thiros", rendered as
  plain text beneath the Reset button — no `Msg` variant or
  interactivity needed, satisfying requirements.md's on-page
  attribution criterion.
- Next to that line, a CC0 badge image (`<img>`), vendored locally as
  `integrator/cc-zero.svg` (fetched once from the official Creative
  Commons mirror at
  `https://mirrors.creativecommons.org/presskit/buttons/80x15/svg/cc-zero.svg`,
  not hotlinked at runtime) and copied into the Trunk build via
  `<link data-trunk rel="copy-file" href="cc-zero.svg">` in
  `index.html`. The `<img src="cc-zero.svg">` uses a plain relative
  path (no leading slash), so the browser resolves it against the
  page's own URL — this works unmodified whether the page is served
  from `/` (local `trunk serve`) or a subpath like `/integrator/`
  (GitHub Pages), with no `public-url`-specific logic needed anywhere
  in the Rust code. The image is wrapped in an
  `<a href="https://creativecommons.org/publicdomain/zero/1.0/">`
  linking to the canonical CC0 1.0 Universal deed, opened in a new tab
  (`target="_blank" rel="noopener noreferrer"`, standard practice for
  outbound links so the simulation isn't navigated away from). Static
  markup, no `Msg` variant.

Each slider's `min`/`max` bounds the value to a physically sensible
range, so the UI itself cannot construct an invalid parameter. Bounds
(subject to adjustment during implementation if they feel wrong in
practice):

| Parameter     | Min  | Max |
|---------------|------|-----|
| Gravity       | 0    | 20  |
| Cart mass     | 0.1  | 10  |
| Pole mass     | 0.1  | 10  |
| Pole length   | 0.1  | 5   |
| Friction (μ)  | 0    | 2   |
| Pivot damping | 0    | 2   |

## Event Log

`log` is a `VecDeque<String>` capped at `MAX_LOG_ENTRIES` (50) entries;
pushing past the cap pops the oldest entry from the front. Entries are
pushed on `Msg::Push`, `Msg::SetX`, and `Msg::Reset` — never on
`Msg::Tick`, which would spam the log every frame. `view_log` renders
entries oldest-first inside a fixed-height, `overflow-y: auto` panel, so
the newest entry lands at the bottom where it's already visible,
satisfying requirements.md's "newest entry visible without additional
scrolling" criterion.

This is transient, in-memory state; it is never persisted, matching
requirements.md's Out of Scope note on data logging.

## Error Handling

Same posture as the solver and simulator: infallible, pure state
transitions. Slider `min`/`max` attributes are the only guard against
invalid physics parameters — there is no runtime validation layer,
consistent with this being a trusted, single-user local tool rather than
a service handling untrusted input.

## Verification Approach

Automated testing is limited for a rendered UI in this project's scope.
Verification is manual: run `cargo xtask web serve`, open the page in a
browser, and confirm the acceptance criteria in
`specs/web/requirements.md` — the cart and pole render and animate, the
layout matches the controls/viewer/log arrangement above, sliders and
buttons visibly affect the simulation, and the log records events as
they happen. `wasm-bindgen-test` exists for headless browser testing but
is not adopted here, matching the learning-project scope already applied
to the server component's Out of Scope section.
