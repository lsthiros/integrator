// SPDX-License-Identifier: CC0-1.0

use std::ops::{Add, Mul};

/// A trait for dynamical systems that can be integrated by the RK4 solver.
///
/// Implementors must provide a State type that can be added and scaled,
/// and a method to compute the derivative of the state.
pub trait DynamicalSystem {
    /// The state type of the system. Must support addition and multiplication by f64.
    type State: Copy + Add<Output = Self::State> + Mul<f64, Output = Self::State>;

    /// Compute the derivative of the state at a given point.
    /// Returns d(state)/dt.
    fn derivative(&self, state: Self::State) -> Self::State;
}

/// Perform one step of RK4 integration for a dynamical system.
///
/// Given a system, an initial state, and a time step duration dt,
/// returns the evolved state after one integration step.
///
/// The RK4 method evaluates the system's derivative at four strategic
/// points during the step and combines them with weighted averaging to
/// achieve 4th-order accuracy.
pub fn step<S: DynamicalSystem>(system: &S, state: S::State, dt: f64) -> S::State {
    // k1: derivative at the current state
    let k1 = system.derivative(state);

    // k2: derivative at midpoint, estimated using k1
    let state_mid1 = state + k1 * (dt / 2.0);
    let k2 = system.derivative(state_mid1);

    // k3: derivative at midpoint, refined using k2
    let state_mid2 = state + k2 * (dt / 2.0);
    let k3 = system.derivative(state_mid2);

    // k4: derivative at endpoint estimate, using k3
    let state_end = state + k3 * dt;
    let k4 = system.derivative(state_end);

    // Weighted combination of derivatives
    let weighted_sum = k1 + k2 * 2.0 + k3 * 2.0 + k4;
    state + weighted_sum * (dt / 6.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple test system that exhibits exponential decay: dy/dt = -y
    #[derive(Copy, Clone)]
    struct Decay;

    impl DynamicalSystem for Decay {
        type State = f64;

        fn derivative(&self, state: f64) -> f64 {
            -state
        }
    }

    #[test]
    fn test_rk4_convergence_order() {
        let system = Decay;
        let y0: f64 = 1.0;
        let t_total: f64 = 1.0;

        // Closed-form solution: y(t) = y0 * e^(-t)
        let y_exact = y0 * (-t_total as f64).exp();

        // Integrate at step size h
        let h: f64 = 0.1;
        let n_steps = (t_total / h).round() as usize;
        let mut y_h = y0;
        for _ in 0..n_steps {
            y_h = step(&system, y_h, h);
        }
        let error_h = (y_h - y_exact).abs();

        // Integrate at step size h/2
        let h_half = h / 2.0;
        let n_steps_half = (t_total / h_half).round() as usize;
        let mut y_h_half = y0;
        for _ in 0..n_steps_half {
            y_h_half = step(&system, y_h_half, h_half);
        }
        let error_h_half = (y_h_half - y_exact).abs();

        // For 4th-order convergence, when step size is halved,
        // error should be multiplied by (1/2)^4 = 1/16
        let convergence_rate = error_h / error_h_half;

        // Check that convergence rate is within a factor of 2 of 16
        // (to account for higher-order terms and numerical precision)
        assert!(
            convergence_rate > 8.0 && convergence_rate < 32.0,
            "convergence_rate {} not in expected range [8, 32]",
            convergence_rate
        );
    }
}
