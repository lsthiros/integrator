<!-- SPDX-License-Identifier: CC0-1.0 -->

# Yew — Cursory Research Notes

## What It Is

Yew is a Rust framework for building client-side web front-ends that compile to
WebAssembly. Its component model mirrors React and Elm. The `html!` macro
provides JSX-like templating using Rust syntax and expressions directly, with
type checking at compile time.

## Component Styles

Yew offers two approaches:

**Function components** use hooks—`use_state`, `use_effect`,
`use_node_ref`—analogous to React. They suit straightforward UI logic.

**Struct components** implement the `Component` trait with `Message`,
`Properties`, `create`, `update`, `view`, and `rendered`. This approach is more
explicit and closer to Elm's message-passing update loop.

## Events and Callbacks

The `Callback<T>` type wires directly into `html!` attributes such as `onclick`
and `oninput`. Standard HTML elements—`<input type="range">` for sliders,
`<button>` for discrete actions—work natively; event callbacks parse values from
the event target.

## Rendering and Updates

A component re-renders when its state changes: in function components, via a
state setter; in struct components, when `update` returns `true` after
dispatching a `Msg`. Yew itself provides only the component lifecycle, not a
game loop or ticking clock. To drive recurring updates every N milliseconds, use
the `gloo` crate family (`gloo-timers` for `Interval` and `Timeout`,
`gloo-render` for `requestAnimationFrame`). These schedule callbacks that
dispatch `Msg` back into the component on each tick to advance state.

## Build Tooling

Trunk is the de facto standard for building and bundling Yew applications. It
compiles the crate to `wasm32-unknown-unknown`, runs `wasm-bindgen`, bundles a
generated `index.html`, and serves locally during development.

## Implications for This Project

Per the Rustonomicon decision, the web UI lives as a module in the unified
`integrator` crate, compiling to `wasm32-unknown-unknown` alongside the
simulator and solver.

Struct components appear better suited than function-component hooks here: the
simulator already has explicit state and an "advance one tick" function
(`Simulator::advance`), which maps directly onto the `Msg` → `update` → `view`
cycle. Function components would require lifting that logic into hooks for no
architectural gain.

Trunk integration with this project's `xtask` automation (see AGENTS.md's
Rustonomicon appendix) remains a `design.md`-level question.

## Sources

- https://yew.rs
- https://docs.rs/yew

**Caveat:** The component-model and callback details come directly from Yew's
official documentation. The gloo crate names and Trunk details reflect general
familiarity with the Rust WASM ecosystem rather than specific pages fetched
during this research pass. Verify exact API names (e.g.,
`gloo::timers::callback::Interval`) independently before relying on them in
design decisions.
