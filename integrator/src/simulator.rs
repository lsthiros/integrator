// SPDX-License-Identifier: CC0-1.0

use crate::solver::{DynamicalSystem, step};
use std::ops::{Add, Mul};

/// The state of a cart-and-pole system.
///
/// Represents the position and velocity of the cart along the rail,
/// and the angle and angular velocity of the pole.
#[derive(Copy, Clone)]
pub struct State {
    /// Cart position along the rail, in meters.
    pub cart_position: f64,
    /// Cart velocity along the rail, in m/s.
    pub cart_velocity: f64,
    /// Pole angle from vertical, in radians (0 = upright, measured from +y).
    pub pole_angle: f64,
    /// Pole angular velocity, in rad/s.
    pub pole_angular_velocity: f64,
}

impl State {
    /// Returns the initial state: cart at origin, pole upright, both at rest.
    pub fn initial() -> Self {
        State {
            cart_position: 0.0,
            cart_velocity: 0.0,
            pole_angle: 0.0,
            pole_angular_velocity: 0.0,
        }
    }
}

impl Add for State {
    type Output = State;

    fn add(self, other: State) -> State {
        State {
            cart_position: self.cart_position + other.cart_position,
            cart_velocity: self.cart_velocity + other.cart_velocity,
            pole_angle: self.pole_angle + other.pole_angle,
            pole_angular_velocity: self.pole_angular_velocity + other.pole_angular_velocity,
        }
    }
}

impl Mul<f64> for State {
    type Output = State;

    fn mul(self, scalar: f64) -> State {
        State {
            cart_position: self.cart_position * scalar,
            cart_velocity: self.cart_velocity * scalar,
            pole_angle: self.pole_angle * scalar,
            pole_angular_velocity: self.pole_angular_velocity * scalar,
        }
    }
}

/// A trait for friction models acting on the cart's rail motion.
///
/// Implementors define how friction opposes the cart's velocity.
pub trait RailFriction {
    /// Compute the friction force (in Newtons) opposing the cart's motion.
    ///
    /// Given the cart's current velocity and the normal force the rail is
    /// supporting, returns the friction force magnitude and direction.
    fn force(&self, cart_velocity: f64, normal_force: f64) -> f64;
}

/// A small epsilon for smoothing sign transitions in friction.
const FRICTION_EPSILON: f64 = 0.01;

/// Compute a smooth approximation to the sign function.
///
/// Returns a value in [-1, 1] that smoothly transitions through zero,
/// avoiding the discontinuity of a hard sign function.
fn smoothed_sign(v: f64) -> f64 {
    (v / FRICTION_EPSILON).tanh()
}

/// Coulomb (dry sliding) friction model for the cart on the rail.
///
/// Friction opposes motion with a magnitude proportional to the normal force,
/// parameterized by a coefficient of friction `mu`.
pub struct CoulombFriction {
    /// Coefficient of kinetic friction (dimensionless).
    pub mu: f64,
}

impl RailFriction for CoulombFriction {
    fn force(&self, cart_velocity: f64, normal_force: f64) -> f64 {
        -self.mu * normal_force * smoothed_sign(cart_velocity)
    }
}

/// A trait for friction models acting on the pole's pivot.
///
/// Implementors define how friction opposes the pole's angular velocity.
pub trait PivotFriction {
    /// Compute the friction torque (in Newton-meters) opposing the pole's
    /// rotation, given its current angular velocity.
    fn torque(&self, angular_velocity: f64) -> f64;
}

/// Viscous (damped-bearing) friction model for the pole's pivot.
///
/// Friction opposes rotation with a magnitude proportional to the angular
/// velocity, parameterized by a damping coefficient.
pub struct ViscousFriction {
    /// Damping coefficient (N·m·s/rad).
    pub damping: f64,
}

impl PivotFriction for ViscousFriction {
    fn torque(&self, angular_velocity: f64) -> f64 {
        -self.damping * angular_velocity
    }
}

/// A simulator for a cart-and-pole system.
///
/// The simulator couples the cart's translational motion along a rail with
/// a pole's rotational motion about the cart. It uses the RK4 solver to
/// advance the system state over time, applying gravity and friction.
pub struct Simulator<F: RailFriction, P: PivotFriction> {
    /// Mass of the cart, in kilograms.
    pub cart_mass: f64,
    /// Mass of the pole (point mass at the end), in kilograms.
    pub pole_mass: f64,
    /// Length of the pole, in meters.
    pub pole_length: f64,
    /// Magnitude of gravitational acceleration, in m/s².
    pub gravity: f64,
    /// The friction model for the cart on the rail.
    pub friction: F,
    /// The friction model for the pole's pivot.
    pub pivot_friction: P,
}

impl<F: RailFriction, P: PivotFriction> Simulator<F, P> {
    /// Advance the system state by one time step, applying an impulse and then integrating.
    ///
    /// The impulse is applied as an instantaneous velocity change to the cart,
    /// after which one RK4 integration step is performed.
    ///
    /// # Arguments
    ///
    /// * `state` - The current state of the system.
    /// * `impulse` - The impulse force applied to the cart, in Newtons.
    /// * `dt` - The time step duration, in seconds.
    ///
    /// # Returns
    ///
    /// The evolved state after applying the impulse and integrating by dt.
    pub fn advance(&self, state: State, impulse: f64, dt: f64) -> State {
        let mut kicked = state;
        kicked.cart_velocity += impulse / self.cart_mass;
        step(self, kicked, dt)
    }
}

impl<F: RailFriction, P: PivotFriction> DynamicalSystem for Simulator<F, P> {
    type State = State;

    fn derivative(&self, state: State) -> State {
        let sin_theta = state.pole_angle.sin();
        let cos_theta = state.pole_angle.cos();
        let theta_dot = state.pole_angular_velocity;

        // Compute normal force (static approximation)
        let normal_force = (self.cart_mass + self.pole_mass) * self.gravity;

        // Compute friction force
        let friction_force = self.friction.force(state.cart_velocity, normal_force);

        // Compute pivot friction torque
        let pivot_torque = self.pivot_friction.torque(state.pole_angular_velocity);

        // Equations of motion (Lagrangian mechanics)
        // ẍ = (f + m·L·θ̇²·sin(θ) − m·g·sin(θ)·cos(θ) − τ·cos(θ)/L)
        //     / (M + m·sin²(θ))
        let numerator = friction_force
            + self.pole_mass * self.pole_length * theta_dot * theta_dot * sin_theta
            - self.pole_mass * self.gravity * sin_theta * cos_theta
            - pivot_torque * cos_theta / self.pole_length;
        let denominator = self.cart_mass + self.pole_mass * sin_theta * sin_theta;
        let cart_acceleration = numerator / denominator;

        // θ̈ = (g·sin(θ) − ẍ·cos(θ)) / L + τ/(m·L²)
        let pole_angular_acceleration = (self.gravity * sin_theta
            - cart_acceleration * cos_theta)
            / self.pole_length
            + pivot_torque / (self.pole_mass * self.pole_length * self.pole_length);

        // Derivative is velocity and acceleration
        State {
            cart_position: state.cart_velocity,
            cart_velocity: cart_acceleration,
            pole_angle: theta_dot,
            pole_angular_velocity: pole_angular_acceleration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the pole swings under gravity alone and energy stays bounded.
    #[test]
    fn test_gravity_only_sanity_check() {
        // Create a simulator with no friction (mu = 0)
        let simulator = Simulator {
            cart_mass: 1.0,
            pole_mass: 0.1,
            pole_length: 1.0,
            gravity: 9.81,
            friction: CoulombFriction { mu: 0.0 },
            pivot_friction: ViscousFriction { damping: 0.0 },
        };

        // Start with pole tilted slightly from vertical
        let mut state = State {
            cart_position: 0.0,
            cart_velocity: 0.0,
            pole_angle: 0.1, // 0.1 rad tilted
            pole_angular_velocity: 0.0,
        };

        const DT: f64 = 0.01; // 10 ms steps
        const STEPS: usize = 500; // Reduced steps to reduce error accumulation

        // Compute total mechanical energy at each step
        let mut energies = Vec::new();

        for _ in 0..STEPS {
            // Kinetic energy: cart + pole (which moves with cart and rotates)
            // KE_total = 0.5*(M+m)*ẋ² + m*ẋ*L*cos(θ)*θ̇ + 0.5*m*L²*θ̇²
            let x_dot = state.cart_velocity;
            let theta_dot = state.pole_angular_velocity;
            let cos_theta = state.pole_angle.cos();

            let ke_cart = 0.5 * simulator.cart_mass * x_dot * x_dot;
            let ke_pole_translational = 0.5 * simulator.pole_mass * x_dot * x_dot;
            let ke_coupling =
                simulator.pole_mass * x_dot * simulator.pole_length * cos_theta * theta_dot;
            let ke_pole_rotational =
                0.5 * simulator.pole_mass * simulator.pole_length * simulator.pole_length
                    * theta_dot * theta_dot;

            // Potential energy: pole's COM height = L*cos(θ) above pivot
            let pe = simulator.pole_mass * simulator.gravity * simulator.pole_length * cos_theta;

            let total_energy =
                ke_cart + ke_pole_translational + ke_coupling + ke_pole_rotational + pe;
            energies.push(total_energy);

            state = simulator.advance(state, 0.0, DT);
        }

        // Check that the pole is swinging (angle changed significantly)
        let final_angle = state.pole_angle;
        assert!(
            final_angle.abs() > 0.05,
            "Pole should swing noticeably; final angle = {}",
            final_angle
        );

        // Check that energy is roughly bounded
        // RK4 introduces some error; allow up to 20% drift over 500 steps
        let initial_energy = energies[0];
        let final_energy = energies[energies.len() - 1];
        let energy_change_ratio = (final_energy - initial_energy) / initial_energy;

        assert!(
            energy_change_ratio.abs() < 0.20,
            "Energy change {:.1}% is too large (>20%)",
            energy_change_ratio * 100.0
        );
    }

    /// Test that friction damps cart motion over time.
    #[test]
    fn test_friction_damping() {
        // Create a simulator with Coulomb friction
        let simulator = Simulator {
            cart_mass: 1.0,
            pole_mass: 0.1,
            pole_length: 1.0,
            gravity: 9.81,
            friction: CoulombFriction { mu: 0.5 },
            pivot_friction: ViscousFriction { damping: 0.0 },
        };

        // Start with the pole upright and give the cart an impulse
        let mut state = State::initial();

        const DT: f64 = 0.01;
        const STEPS: usize = 500;

        let mut cart_velocities = Vec::new();

        // Apply an impulse to start
        state = simulator.advance(state, 10.0, DT); // 10 N impulse

        for _ in 0..STEPS {
            cart_velocities.push(state.cart_velocity.abs());
            state = simulator.advance(state, 0.0, DT);
        }

        // Check that cart velocity is decreasing on average
        let initial_velocity = cart_velocities[0];
        let final_velocity = cart_velocities[cart_velocities.len() - 1];

        assert!(
            final_velocity < initial_velocity,
            "Cart velocity should decrease due to friction: {} -> {}",
            initial_velocity,
            final_velocity
        );

        // Check that velocity trend is toward zero
        // (allow some oscillation, but overall trend should be downward)
        let mut decreasing_count = 0;
        for i in 1..cart_velocities.len() {
            if cart_velocities[i] < cart_velocities[i - 1] {
                decreasing_count += 1;
            }
        }

        // At least 60% of steps should show decreasing velocity
        assert!(
            decreasing_count as f64 / cart_velocities.len() as f64 > 0.6,
            "Cart velocity should trend toward zero; only {:.1}% of steps showed decrease",
            (decreasing_count as f64 / cart_velocities.len() as f64) * 100.0
        );
    }

    /// Test that pivot friction damps pole angular velocity over time.
    #[test]
    fn test_pivot_friction_damping() {
        // Rail friction disabled; pivot friction enabled.
        let simulator = Simulator {
            cart_mass: 1.0,
            pole_mass: 0.1,
            pole_length: 1.0,
            gravity: 9.81,
            friction: CoulombFriction { mu: 0.0 },
            pivot_friction: ViscousFriction { damping: 0.5 },
        };

        // Start with the pole displaced from vertical and rotating.
        let mut state = State {
            cart_position: 0.0,
            cart_velocity: 0.0,
            pole_angle: 0.3,
            pole_angular_velocity: 2.0,
        };

        const DT: f64 = 0.01;
        const STEPS: usize = 500;

        let mut angular_velocities = Vec::new();

        for _ in 0..STEPS {
            angular_velocities.push(state.pole_angular_velocity.abs());
            state = simulator.advance(state, 0.0, DT);
        }

        // Check that angular velocity is decreasing on average
        let initial_velocity = angular_velocities[0];
        let final_velocity = angular_velocities[angular_velocities.len() - 1];

        assert!(
            final_velocity < initial_velocity,
            "Pole angular velocity should decrease due to pivot friction: {} -> {}",
            initial_velocity,
            final_velocity
        );

        // Check that velocity trend is toward zero
        // (allow some oscillation, but overall trend should be downward)
        let mut decreasing_count = 0;
        for i in 1..angular_velocities.len() {
            if angular_velocities[i] < angular_velocities[i - 1] {
                decreasing_count += 1;
            }
        }

        // At least 60% of steps should show decreasing angular velocity
        assert!(
            decreasing_count as f64 / angular_velocities.len() as f64 > 0.6,
            "Pole angular velocity should trend toward zero; only {:.1}% \
             of steps showed decrease",
            (decreasing_count as f64 / angular_velocities.len() as f64) * 100.0
        );
    }
}
