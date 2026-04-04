// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or any later version.

#![no_std] // Suitable for embedded systems without standard library

//! # so2nostd
//!
//! Discrete-time Second-Order (SO2) controller for embedded systems.
//! Robust, physics-based, `no_std` Rust implementation.

#[cfg(feature = "f32")]
pub type Float = f32;
#[cfg(not(feature = "f32"))]
pub type Float = f64;

/// Core SO2 Controller structure
#[derive(Debug, Clone, Copy)]
pub struct So2Controller {
    /// Natural frequency ω_n (rad/s)
    pub w_n: Float,
    /// Damping ratio ζ (1.0 = critically damped)
    pub zeta: Float,
    /// Current state value
    y: Float,
    /// Previous state value
    y_prev: Float,
    /// Target setpoint
    pub setpoint: Float,
    /// Static gain
    pub gain: Float,
    /// Optional max velocity for safety (units/sec)
    pub max_velocity: Option<Float>,
    /// Optional max acceleration for safety (units/sec²)
    pub max_acceleration: Option<Float>,
}

impl So2Controller {
    /// Create a new SO2 controller
    #[inline(always)]
    pub fn new(
        w_n: Float,
        zeta: Float,
        initial_value: Float,
        gain: Float,
    ) -> Self {
        Self {
            w_n,
            zeta,
            y: initial_value,
            y_prev: initial_value,
            setpoint: initial_value,
            gain,
            max_velocity: None,
            max_acceleration: None,
        }
    }

    /// Update the controller state for given input and timestep
    #[inline(always)]
    pub fn update(&mut self, input: Float, dt: Float) -> Float {
        if !dt.is_finite() || dt <= 0.0 {
            return self.y;
        }

        // Manual clamp for maximum no_std portability
        let safe_dt = if dt < 1e-6 { 1e-6 } else if dt > 0.1 { 0.1 } else { dt };

        let a = self.w_n * self.w_n;
        let b = 2.0 * self.zeta * self.w_n;

        let dy = (self.y - self.y_prev) / safe_dt;
        let mut d2y = self.gain * a * (self.setpoint - self.y) - b * dy - a * self.y;

        if let Some(max_a) = self.max_acceleration {
            if d2y > max_a { d2y = max_a; }
            else if d2y < -max_a { d2y = -max_a; }
        }

        let mut next_y = self.y + dy * safe_dt + 0.5 * d2y * safe_dt * safe_dt;

        if let Some(max_v) = self.max_velocity {
            let vel = (next_y - self.y) / safe_dt;
            if vel > max_v { next_y = self.y + max_v * safe_dt; }
            else if vel < -max_v { next_y = self.y - max_v * safe_dt; }
        }

        self.y_prev = self.y;
        self.y = next_y;
        self.y
    }
    /// Set a new target setpoint
    #[inline(always)]
    pub fn set_target(&mut self, target: Float) {
        self.setpoint = target;
    }

    /// Reset controller internal states
    #[inline(always)]
    pub fn reset(&mut self, value: Float) {
        self.y = value;
        self.y_prev = value;
        self.setpoint = value;
    }

    /// Optional: set max velocity limit
    #[inline(always)]
    pub fn set_max_velocity(&mut self, max_v: Float) {
        self.max_velocity = Some(max_v);
    }

    /// Optional: set max acceleration limit
    #[inline(always)]
    pub fn set_max_acceleration(&mut self, max_a: Float) {
        self.max_acceleration = Some(max_a);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_so2_stability() {
        let mut so2 = So2Controller::new(20.0, 0.5, 0.0, 1.0);
        let dt = 0.005;
        let mut y = 0.0;
        for _ in 0..200 { y = so2.update(10.0, dt); }
        assert!((y - 10.0).abs() < 0.5);
    }

    #[test]
    fn test_zero_dt_integrity() {
        let mut so2 = So2Controller::new(10.0, 1.0, 5.0, 1.0);
        let output = so2.update(10.0, 0.0);
        assert_eq!(output, 5.0);
    }

    #[test]
    fn test_setpoint_tracking() {
        let mut so2 = So2Controller::new(10.0, 0.7, 0.0, 1.0);
        so2.set_target(1.0);
        let mut y = 0.0;
        for _ in 0..100 { y = so2.update(0.0, 0.01); }
        assert!((y - 1.0).abs() < 0.5);
    }
}