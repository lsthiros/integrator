<!-- SPDX-License-Identifier: CC0-1.0 -->

# Integrator

## Abstract

The purpose of this document is to describe the study, creation, and testing of
a rust-based physics simulator written for the purposes of learning. It
describes a physics simulator that shall be coded with the help of an AI agent
following several restrictions and practices.

## Project

### Background

Physics simulators can provide a base for understanding the mathematics and
behaviors of dynamic systems, both linear and nonlinear in nature. An engineer
wishing to learn more about these systems may use software solvers with
graphical rendering capabilities to obtain intuitive understandings of these
systems.

Runge Katta, or RK4, is a method of numerical integration that can be used to
solve the ongoing and dynamic state of a physical system.

Rust is the programming language chosen for this project. It will be compiled
into wasm and presented via a simple HTTP server so that it may be observed
from somewhere other than this headless host.

### Deliverables or Goals

The goal of this project is to provide a rust-based application that can render
a dynamical system in the form of a "cart and pole". The cart and pole system
will be rendered in a web application structured as a basic simulator with
tunable values.

### High Level UX and Requirements

The application will present as a single page application viewable via a web
browser on a host other than this one. It will present the "cart" on a "rail"a
with a "pole". A user will be able to interract with the application by
providing an impulse or "push" to the cart and pole system.

### Low Level Organization

The application will be split into components with specific responsibilities.
It will be up to the agent to define the APIs.

The first component will be the integrator, or solver. It shall contain the
core RK4 implementation. In order to support a variety of dynamical systems,
its API shall support opaque dynamic system "objects" that can provide the
required derrivative outputs on demand. For example, a Polynomial "object"
will provide an interface to "solve" for a given input, wherein `x` is provided
while also providing first and second derivative outputs.

The second component shall be the simulator, which uses the primitives provided
by the first component to create a representation of a cart and pole system. It
is possible, but not necessary, that a legrangian representation by used to
denote the state. The simulator shall support the ability to be provided with
an input at each "tick" that shall be treated as a impulse force on the cart
upon the rail to which it is constrained. Gravity shall be included in this
simulation.

The third component shall be a web based UI. The web based UI will contain
a render window, and appropriate controls to interract with the physics
simulation. It shall contain the appropriate HTML needed to drive the simulator.

An optional component shall be a simple wrapper around an HTTP server used to
vend the web based UI. It shall be written in rust.

## Appendix A: Spec-Driven Development Skill

### Survey of official references

- **Microsoft/GitHub — [github/spec-kit](https://github.com/github/spec-kit)**: a
  slash-command-driven workflow — `/speckit.constitution` →
  `/speckit.specify` → `/speckit.plan` → `/speckit.tasks` →
  `/speckit.implement` (with optional `/speckit.clarify`, `/speckit.analyze`,
  `/speckit.checklist` along the way). Artifacts (`constitution.md`,
  `spec.md`, `plan.md`, `tasks.md`) live under `.specify/` and are treated as
  the versioned source of truth; a separate `.github/` folder holds
  agent-specific command bindings.
- **Amazon — [kirodotdev/Kiro](https://github.com/kirodotdev/Kiro)**: a
  per-feature, three-file spec under `.kiro/specs/<feature>/` —
  `requirements.md` (user stories + acceptance criteria), `design.md`
  (architecture, sequence diagrams, error handling), `tasks.md` (discrete,
  dependency-ordered tasks, groupable into parallel "waves"). Supplemented by
  "steering" files (persistent project context/rules) and "hooks"
  (event-triggered automation).
- **Anthropic — [anthropics/skills](https://github.com/anthropics/skills)**:
  no packaged SDD skill exists in the official repo. Anthropic instead
  publishes the Agent Skills spec itself — a `SKILL.md` with YAML
  frontmatter plus instructions — i.e., the mechanism for authoring a custom
  skill such as this one, rather than a prescribed spec-driven methodology.

### Derived process for this project

AGENTS.md already serves as this project's constitution, so that step is not
duplicated. Kiro's three-file naming is adopted (clearer than Spec Kit's
`spec`/`plan`) with Spec Kit's discipline of not writing code until the
preceding artifact exists, scaled down for a single-developer-plus-agent
project (no issue-tracker conversion, no automated task "waves").

For each component (integrator, simulator, web UI, HTTP server) or
significant feature, before code is written:

1. `requirements.md` — what and why: user-facing behavior, inputs/outputs,
   acceptance criteria. No implementation detail.
2. `design.md` — how: API/interfaces, data flow, chosen algorithms (e.g. the
   RK4 step signature), error handling, and how it interfaces with sibling
   components.
3. `tasks.md` — a checklist of small, independently verifiable
   implementation tasks derived from `design.md`, ordered by dependency.

These live under `specs/<component>/` at the repo root, e.g.
`specs/integrator/requirements.md`. Implementation does not begin until
`requirements.md` and `design.md` have been reviewed and agreed with the
user; deviations discovered during implementation are reflected back into
those files rather than left implicit in the code.

### Delegating to smaller models

Every unit of work — drafting `requirements.md`/`design.md`/`tasks.md`
just as much as writing code — should be handed to the least capable agent
that can plausibly do it. Escalate capability only when the smaller model
demonstrably can't cope; don't default to the main agent out of convenience.

- **Design authority stays with the main agent.** Smaller models delegated a
  `tasks.md` item may implement it, but may not change `requirements.md` or
  `design.md`, choose a different approach than the one those files
  describe, or introduce new public interfaces. If a task can't be
  completed as specified, that's a failure to report, not a license to
  redesign.
- **`tasks.md` is check-boxes-only.** A delegated implementer may flip a
  task's `- [ ]` to `- [x]` as it completes that task, and do nothing else
  to the file — no rewording, reordering, adding, or removing tasks. Any
  other change to `tasks.md` belongs to the main agent, same as
  `requirements.md` and `design.md`.
- **Three-strikes escalation.** If a delegated implementation task fails
  three times (e.g. it fails to compile, fails its own tests, or the
  smaller model reports it cannot proceed), the smaller model stops and
  re-delegates back to the main agent with what it tried and how each
  attempt failed, rather than continuing to retry or improvising around the
  design.
- **No `sudo`, for anyone.** This restriction applies to the main agent as
  much as to any delegated smaller model — no agent in this project may
  invoke `sudo` or otherwise escalate privileges, regardless of capability
  level. If a task genuinely appears to need `sudo`, the main agent stops
  and asks the user rather than running it or working around it.

## Appendix B: The Rustonomicon (Crate and Library Layout)

### Default to a single crate

The components described under Low Level Organization (integrator,
simulator, web UI, HTTP server) should start as **modules within one
crate**, not separate crates. A module boundary is cheap to draw and cheap
to undo; a crate boundary is not. Only split a module out into its own
crate when there is a concrete forcing reason, for example:

- it targets a different compilation target than the rest (e.g. the web UI
  compiles to `wasm32-unknown-unknown` while a native HTTP server does
  not),
- it needs to be published or reused independently of this project, or
- it's needed to break an otherwise unavoidable dependency cycle.

"It's a separate concern" is not by itself sufficient justification — that's
what modules are for. When a split is proposed, the reason belongs in that
component's `design.md` (see the Spec-Driven Development appendix above),
not left implicit.

### Decision: one crate, plus two forced exceptions

Applying the above, this project has decided: the **integrator (solver),
simulator, and web UI live together as modules in the existing `integrator`
crate**, since all three compile to the same target
(`wasm32-unknown-unknown`) and ship together as one WASM artifact. Only two
things are split out, because both fail the "same compilation target"
test — neither is meant to run in the browser at all:

- **`server/`** — a native binary; its only job is vending the built WASM
  artifact over HTTP to a browser on another host.
- **`xtask/`** — a native, dev-only binary; project automation per the
  `xtask` convention below. It never ships and has no reason to be coupled
  to the WASM build.

Both are laid out **flat** as siblings of `integrator/` at the repository
root under a Cargo workspace — never nested, never wrapped in an extra
`crates/` directory:

```text
integrator/            (repo root; the git repository / workspace root)
├── Cargo.toml          (workspace manifest)
├── integrator/          (unified crate: solver + simulator + web UI; compiles to wasm32-unknown-unknown)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── solver.rs     (RK4 + dynamic system trait)
│       ├── simulator.rs  (cart-and-pole state, uses solver)
│       └── web.rs        (render window + controls, uses simulator)
├── server/              (native HTTP server, serves integrator's wasm build)
│   ├── Cargo.toml
│   └── src/main.rs
└── xtask/               (native dev-only automation; never shipped)
    ├── Cargo.toml
    └── src/main.rs
```

If a future component needs its own compilation target, independent
publishing, or breaks a dependency cycle within the `integrator` crate, it
gets added as another flat sibling here — with the reason recorded in its
`design.md`, per the pattern above.

### Project automation: `xtask`, not `build.rs` or shell scripts

For project-level automation (running the wasm build, copying assets,
serving the UI locally, code generation, etc.), use the community `xtask`
convention rather than a Cargo `build.rs` build script or ad-hoc shell
scripts:

- Automation lives in its own plain binary crate, conventionally named
  `xtask/`, added as a flat sibling crate per the layout above.
- It's invoked as `cargo xtask <command>` via a `.cargo/config.toml` alias,
  and is just regular Rust — debuggable, testable, and portable across
  platforms, unlike shell scripts.
- `build.rs` is reserved for what it's actually for — build-time codegen or
  linking that Cargo needs to know about as part of compiling a specific
  crate. It's not a place for project automation, since it runs implicitly
  on every build and can't be invoked or composed on its own.

This is a Rust community convention (popularized outside any Anthropic,
Microsoft, or Amazon material referenced above, and not part of the
official Rustonomicon at doc.rust-lang.org/nomicon) — noted here on its own
merits, not as something derived from those sources.

## Appendix C: Runge-Kutta (RK4), Simply

RK4 is a way to approximate how a system changes over time when you only know
its instantaneous rate of change (its derivative), not a closed-form formula
for its future state. This project needs it because the cart-and-pole's
position and velocity at the next instant depend on forces (gravity, the
user's push) acting right now, not on some equation you could just plug a
future time into.

The simplest approach, Euler's method, takes the current slope and walks a
straight line forward by one time step. This drifts from the true curve
quickly, especially as the step size grows.

RK4 improves on this by sampling the slope four times per step instead of
once, then combining those samples into a single, more accurate step:

1. **k1** — the slope at the start of the step (the "obvious" guess).
2. **k2** — the slope at the step's midpoint, using k1 to estimate where the
   midpoint is.
3. **k3** — the slope at the midpoint again, this time using k2 for a
   refined midpoint estimate.
4. **k4** — the slope at the end of the step, using k3 to estimate where the
   step ends.

These four slopes are then averaged, with the two midpoint estimates (k2,
k3) weighted twice as heavily as the endpoint estimates (k1, k4), and that
weighted average is used to advance the state by one time step. The result
tracks curved motion far more faithfully than Euler's method at the same
step size, which is why it's the standard choice for simulating dynamical
systems like a cart and pole.

## Appendix D: Agent Writing Style

The agent is forbidden from writing sentences of the form "no X, no Y, no Z" (a bare list of negated nouns/phrases for rhetorical effect). State what's true or what was decided directly instead.
