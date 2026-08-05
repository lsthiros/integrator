// SPDX-License-Identifier: CC0-1.0

//! The web-based UI for the cart-and-pole simulator.
//!
//! Renders the simulation state as inline SVG, drives it on a fixed tick
//! interval, and exposes controls for applying impulses and tuning
//! simulation parameters. See `specs/web/design.md` for the design this
//! module implements.

use crate::simulator::{CoulombFriction, Simulator, State, ViscousFriction};
use gloo_timers::callback::Interval;
use std::collections::VecDeque;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::TargetCast;

/// Interval between simulation ticks, in milliseconds.
const TICK_MS: u32 = 16;
/// Simulation time step corresponding to one tick, in seconds.
const DT: f64 = TICK_MS as f64 / 1000.0;
/// Magnitude of the impulse applied by the push buttons, in Newtons.
const IMPULSE_MAGNITUDE: f64 = 5.0;
/// Maximum number of entries retained in the event log.
const MAX_LOG_ENTRIES: usize = 50;
/// Scale factor converting physical meters to screen pixels.
const PIXELS_PER_METER: f64 = 100.0;
/// X coordinate, in pixels, of the physical origin (cart position 0).
const ORIGIN_PX_X: f64 = 400.0;
/// Y coordinate, in pixels, of the rail (and pole pivot).
const ORIGIN_PX_Y: f64 = 300.0;

/// Width of the viewer's SVG viewBox, in pixels.
const VIEW_WIDTH: f64 = 800.0;
/// Height of the viewer's SVG viewBox, in pixels.
const VIEW_HEIGHT: f64 = 400.0;
/// Width of the rendered cart rectangle, in pixels.
const CART_WIDTH: f64 = 60.0;
/// Height of the rendered cart rectangle, in pixels.
const CART_HEIGHT: f64 = 30.0;

/// Messages that drive the `App` component's `update` cycle.
pub enum Msg {
    /// Advance the simulation by one tick with no applied impulse.
    Tick,
    /// Apply an instantaneous impulse (in Newtons) to the cart.
    Push(f64),
    /// Set the simulator's gravitational acceleration (m/s^2).
    SetGravity(f64),
    /// Set the simulator's cart mass (kg).
    SetCartMass(f64),
    /// Set the simulator's pole mass (kg).
    SetPoleMass(f64),
    /// Set the simulator's pole length (m).
    SetPoleLength(f64),
    /// Set the simulator's coefficient of friction.
    SetFriction(f64),
    /// Set the simulator's pivot (rotational) damping coefficient.
    SetPivotDamping(f64),
    /// Reset the simulation state to its initial conditions.
    Reset,
}

/// The root Yew component for the cart-and-pole web UI.
pub struct App {
    simulator: Simulator<CoulombFriction, ViscousFriction>,
    state: State,
    log: VecDeque<String>,
    _ticker: Interval,
}

impl App {
    /// Append `entry` to the event log, evicting the oldest entry first if
    /// the log is already at capacity.
    fn push_log(&mut self, entry: String) {
        if self.log.len() >= MAX_LOG_ENTRIES {
            self.log.pop_front();
        }
        self.log.push_back(entry);
    }

    /// Build an `oninput` callback that parses the input element's value as
    /// `f64` and wraps it in the given message constructor.
    fn parameter_input(link: &html::Scope<Self>, make_msg: fn(f64) -> Msg) -> Callback<InputEvent> {
        link.callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            make_msg(input.value().parse().unwrap_or(0.0))
        })
    }

    fn view_controls(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let push_left = link.callback(|_| Msg::Push(-IMPULSE_MAGNITUDE));
        let push_right = link.callback(|_| Msg::Push(IMPULSE_MAGNITUDE));
        let reset = link.callback(|_| Msg::Reset);

        let on_gravity = Self::parameter_input(link, Msg::SetGravity);
        let on_cart_mass = Self::parameter_input(link, Msg::SetCartMass);
        let on_pole_mass = Self::parameter_input(link, Msg::SetPoleMass);
        let on_pole_length = Self::parameter_input(link, Msg::SetPoleLength);
        let on_friction = Self::parameter_input(link, Msg::SetFriction);
        let on_pivot_damping = Self::parameter_input(link, Msg::SetPivotDamping);

        html! {
            <div class="controls">
                <div class="control-group">
                    <button onclick={push_left}>{ "Push Left" }</button>
                    <button onclick={push_right}>{ "Push Right" }</button>
                </div>
                <label>
                    { format!("Gravity: {:.2} m/s\u{b2}", self.simulator.gravity) }
                    <input
                        type="range"
                        min="0"
                        max="20"
                        step="0.1"
                        value={self.simulator.gravity.to_string()}
                        oninput={on_gravity}
                    />
                </label>
                <label>
                    { format!("Cart mass: {:.2} kg", self.simulator.cart_mass) }
                    <input
                        type="range"
                        min="0.1"
                        max="10"
                        step="0.1"
                        value={self.simulator.cart_mass.to_string()}
                        oninput={on_cart_mass}
                    />
                </label>
                <label>
                    { format!("Pole mass: {:.2} kg", self.simulator.pole_mass) }
                    <input
                        type="range"
                        min="0.1"
                        max="10"
                        step="0.1"
                        value={self.simulator.pole_mass.to_string()}
                        oninput={on_pole_mass}
                    />
                </label>
                <label>
                    { format!("Pole length: {:.2} m", self.simulator.pole_length) }
                    <input
                        type="range"
                        min="0.1"
                        max="5"
                        step="0.1"
                        value={self.simulator.pole_length.to_string()}
                        oninput={on_pole_length}
                    />
                </label>
                <label>
                    { format!("Friction: {:.2} \u{3bc}", self.simulator.friction.mu) }
                    <input
                        type="range"
                        min="0"
                        max="2"
                        step="0.01"
                        value={self.simulator.friction.mu.to_string()}
                        oninput={on_friction}
                    />
                </label>
                <label>
                    { format!("Pivot damping: {:.2} N\u{b7}m\u{b7}s/rad", self.simulator.pivot_friction.damping) }
                    <input
                        type="range"
                        min="0"
                        max="2"
                        step="0.01"
                        value={self.simulator.pivot_friction.damping.to_string()}
                        oninput={on_pivot_damping}
                    />
                </label>
                <button onclick={reset}>{ "Reset" }</button>
            </div>
        }
    }

    fn view_viewer(&self) -> Html {
        let cart_x = ORIGIN_PX_X + self.state.cart_position * PIXELS_PER_METER;
        let cart_y = ORIGIN_PX_Y;
        let tip_x =
            cart_x + self.simulator.pole_length * PIXELS_PER_METER * self.state.pole_angle.sin();
        let tip_y = ORIGIN_PX_Y
            - self.simulator.pole_length * PIXELS_PER_METER * self.state.pole_angle.cos();

        let view_box = format!("0 0 {VIEW_WIDTH} {VIEW_HEIGHT}");

        html! {
            <div class="viewer">
                <svg viewBox={view_box} width="100%" height="100%">
                    <line
                        x1="0"
                        y1={ORIGIN_PX_Y.to_string()}
                        x2={VIEW_WIDTH.to_string()}
                        y2={ORIGIN_PX_Y.to_string()}
                        stroke="#333"
                        stroke-width="2"
                    />
                    <rect
                        x={(cart_x - CART_WIDTH / 2.0).to_string()}
                        y={(cart_y - CART_HEIGHT / 2.0).to_string()}
                        width={CART_WIDTH.to_string()}
                        height={CART_HEIGHT.to_string()}
                        fill="#44aa77"
                    />
                    <line
                        x1={cart_x.to_string()}
                        y1={cart_y.to_string()}
                        x2={tip_x.to_string()}
                        y2={tip_y.to_string()}
                        stroke="#aa4444"
                        stroke-width="4"
                    />
                </svg>
            </div>
        }
    }

    fn view_log(&self) -> Html {
        html! {
            <div class="log">
                { for self.log.iter().map(|entry| html! { <div class="log-entry">{ entry }</div> }) }
            </div>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let simulator = Simulator {
            cart_mass: 1.0,
            pole_mass: 0.1,
            pole_length: 1.0,
            gravity: 9.81,
            friction: CoulombFriction { mu: 0.1 },
            pivot_friction: ViscousFriction { damping: 0.05 },
        };

        let link = ctx.link().clone();
        let ticker = Interval::new(TICK_MS, move || link.send_message(Msg::Tick));

        App {
            simulator,
            state: State::initial(),
            log: VecDeque::with_capacity(MAX_LOG_ENTRIES),
            _ticker: ticker,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.state = self.simulator.advance(self.state, 0.0, DT);
            }
            Msg::Push(magnitude) => {
                self.state = self.simulator.advance(self.state, magnitude, DT);
                let direction = if magnitude < 0.0 { "left" } else { "right" };
                self.push_log(format!(
                    "Pushed {direction} with magnitude {:.2} N",
                    magnitude.abs()
                ));
            }
            Msg::SetGravity(value) => {
                self.simulator.gravity = value;
                self.push_log(format!("Gravity set to {:.2} m/s\u{b2}", value));
            }
            Msg::SetCartMass(value) => {
                self.simulator.cart_mass = value;
                self.push_log(format!("Cart mass set to {:.2} kg", value));
            }
            Msg::SetPoleMass(value) => {
                self.simulator.pole_mass = value;
                self.push_log(format!("Pole mass set to {:.2} kg", value));
            }
            Msg::SetPoleLength(value) => {
                self.simulator.pole_length = value;
                self.push_log(format!("Pole length set to {:.2} m", value));
            }
            Msg::SetFriction(value) => {
                self.simulator.friction.mu = value;
                self.push_log(format!("Friction coefficient set to {:.2}", value));
            }
            Msg::SetPivotDamping(value) => {
                self.simulator.pivot_friction.damping = value;
                self.push_log(format!("Pivot damping set to {:.2} N\u{b7}m\u{b7}s/rad", value));
            }
            Msg::Reset => {
                self.state = State::initial();
                self.push_log("Simulation reset".to_string());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                { self.view_controls(ctx) }
                <div class="right">
                    { self.view_viewer() }
                    { self.view_log() }
                </div>
            </div>
        }
    }
}

/// Mounts the `App` component to the page's `#app` element. Invoked
/// automatically by the wasm-bindgen runtime when the compiled module is
/// loaded in the browser.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
