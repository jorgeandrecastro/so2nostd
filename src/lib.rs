// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or the License, or
// (at your option) any later version.

#![no_std] // No standard library, suitable for embedded environments
//! # so2nostd
//!
//!  Second-Order (SO2) system implementation for discrete-time control.
//! Designed for high-performance embedded systems requiring stability and precision.

/// Core SO2 Controller structure.
/// Implements a discrete-time second-order transfer function.
/// # Example
///
/// ```
/// use so2nostd::So2Controller;
///
/// // Initialisation : omega_n = 10.0, zeta = 0.7
/// let mut so2 = So2Controller::new(10.0, 0.7, 0.0, 1.0);
/// let output = so2.update(1.0, 0.01);
/// assert!(output >= 0.0);
/// ```

// This optionally uses f32 for reduced memory usage (RP2040, etc.), but defaults to f64.
#[cfg(feature = "f32")]
pub type Float = f32;
// By default, we use f64 for better precision in control calculations.
#[cfg(not(feature = "f32"))]
pub type Float = f64;

pub struct So2Controller {
    /// Natural frequency (omega_n)
    pub w_n: Float,
    /// Damping ratio (zeta)
    pub zeta: Float,
    /// Current state value
    y: Float,
    /// Previous state value
    y_prev: Float,
    /// Target setpoint
    setpoint: Float,
    /// Static gain
    gain: Float,
}

impl So2Controller {
    /// Creates a new SO2 controller instance.
    ///
    /// # Arguments
    /// * `w_n` - Natural frequency in rad/s.
    /// * `zeta` - Damping ratio (1.0 is critically damped).
    /// * `initial_value` - Starting point of the system.
    /// * `gain` - System static gain (usually 1.0).
    pub fn new(w_n: Float, zeta: Float, initial_value: Float, gain: Float) -> Self {
        Self {
            w_n,
            zeta,
            y: initial_value,
            y_prev: initial_value,
            setpoint: initial_value,
            gain,
        }
    }

    /// Computes the next state based on the input and delta time.
    /// Uses a discrete approximation of s² + 2ζωns + ωn².
    ///
    /// # Arguments
    /// * `input` - The driving force or target.
    /// * `dt` - Delta time since the last update.
    pub fn update(&mut self, input: Float, dt: Float) -> Float {
        // Safety check for time steps
        if dt <= 0.0 {
            return self.y;
        }

        // Calculation of coefficients for the second-order differential equation
        // a = w_n^2, b = 2 * zeta * w_n
        let a = self.w_n * self.w_n;
        let b = 2.0 * self.zeta * self.w_n;

        // Current velocity (numerical differentiation)
        let dy = (self.y - self.y_prev) / dt;

        // Acceleration: d2y = gain * a * input - b * dy - a * y
        let d2y = (self.gain * a * input) - (b * dy) - (a * self.y);

        // Numerical integration (Euler-Method)
        let next_y = self.y + (dy * dt) + (0.5 * d2y * dt * dt);

        // State update
        self.y_prev = self.y;
        self.y = next_y;

        self.y
    }

    /// Dynamically update the target setpoint.
    pub fn set_target(&mut self, target: Float) {
        self.setpoint = target;
    }

    /// Reset internal states to a specific value.
    pub fn reset(&mut self, value: Float) {
        self.y = value;
        self.y_prev = value;
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_so2_stability() {
        //Test on a simple step input to verify stability and convergence
        let mut so2 = So2Controller::new(20.0, 0.5, 0.0, 1.0);
        let dt = 0.005;
        let mut current_y = 0.0;

        // Simulate 200 iterations with a step input of 10.0
        for _ in 0..200 {
            current_y = so2.update(10.0, dt);
        }

        // The output should converge close to the input value (10.0) without diverging
        assert!((current_y - 10.0).abs() < 0.5);
    }

    #[test]
    fn test_zero_dt_integrity() {
        let mut so2 = So2Controller::new(10.0, 1.0, 5.0, 1.0);
        let output = so2.update(10.0, 0.0);
        // With zero dt, the output should not change from the initial value
        assert_eq!(output, 5.0);
    }
}
