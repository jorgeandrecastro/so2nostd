# so2nostd

[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL%202.0%2B-blue.svg)](https://www.gnu.org/licenses/gpl-2.0)
[![No Std](https://img.shields.io/badge/no_std-compatible-green.svg)](https://docs.rs/so2nostd)
[![Maintenance](https://img.shields.io/badge/Maintenance-Actively--developed-brightgreen.svg)](https://github.com/jorgeandrecastro/so2nostd)

## Second-Order `no_std` Controller for Embedded Systems

**so2nostd** is a lightweight, high-performance `no_std` Rust crate implementing a discrete-time second-order (SO2) control system. Designed for embedded environments like MCUs (e.g., RP2040), it provides stable, physics-based dynamics using Euler integration. 
# Update Version 0.2.1
#![forbid(unsafe_code)] for safety and opt-level = 3 for speed 

GPL-2.0-or-later licensed to ensure community protection against privatization. Optimized for minimal footprint and maximal reliability.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Quickstart](#quickstart)
- [API Reference](#api-reference)
- [Performance & Optimization](#performance--optimization)
- [Testing](#testing)
- [License](#license)

## 🚀 Features
- ✅ **Pure `no_std`**: Zero standard library dependencies, perfect for bare-metal/RTOS.
- ⚡ **Flexible Floating-Point**: `f64` (default) or `f32` feature for memory-constrained devices.
- 🔧 **Size-Optimized**: Compatible with `opt-level="z"`, LTO, and `strip = true`.
- 🛡️ **Numerical Safety**: Handles `dt <= 0`, NaN/Inf, and prevents divergence.
- 📈 **Physics-Based SO2**: Models natural frequency (`ω_n`), damping ratio (`ζ`), and static gain.
- 🎯 **Setpoint Tracking**: Smooth convergence toward `setpoint`.
- ⛑️ **Optional Safety Limits**: `max_velocity` and `max_acceleration` for embedded safety constraints.

## 🛠️ Installation

Add to your `Cargo.toml`:

````toml
[dependencies]
so2nostd = "0.2.1"


f32 for embedded (e.g., Cortex-M):

so2nostd = {version="0.2.1", features = ["f32"] }

cargo add so2nostd

Build with optimizations:

cargo build --release
````

# 🚀 Quickstart

````rust
use so2nostd::So2Controller;

fn main() {
    // ω_n=10 rad/s, ζ=0.7 (underdamped), initial=0.0, gain=1.0
    let mut controller = So2Controller::new(10.0, 0.7, 0.0, 1.0);

    controller.set_target(1.0); // Desired setpoint
    controller.set_max_velocity(0.5); // Optional velocity limit
    controller.set_max_acceleration(10.0); // Optional acceleration limit

    let dt = 0.01; // 10ms timestep

    // Run controller update loop
    for _ in 0..100 {
        let output = controller.update(0.0, dt);
        // output smoothly approaches setpoint following SO2 dynamics
    }
}
````


# 📚 API Reference


| Method                 | Signature                                                                                | Description                                               |
| ---------------------- | ---------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| `new`                  | `So2Controller::new(w_n: Float, zeta: Float, initial_value: Float, gain: Float) -> Self` | Creates a new controller instance.                        |
| `update`               | `&mut self.update(input: Float, dt: Float) -> Float`                                     | Updates system state toward `setpoint`. Safe for dt <= 0. |
| `set_target`           | `&mut self.set_target(target: Float)`                                                    | Updates the internal setpoint.                            |
| `reset`                | `&mut self.reset(value: Float)`                                                          | Resets states (`y`, `y_prev`, `setpoint`) to `value`.     |
| `set_max_velocity`     | `&mut self.set_max_velocity(max_v: Float)`                                               | Optional: clamps max velocity.                            |
| `set_max_acceleration` | `&mut self.set_max_acceleration(max_a: Float)`                                           | Optional: clamps max acceleration.                        |





Type: Float = f64 (default) or f32 with feature flag.

Public fields: w_n, zeta, setpoint, gain (inspectable/tunable).

⚡ Performance & Optimization
Binary Size: Minimal, optimized with opt-level=3, LTO, strip = true.
CPU Cost: Constant time O(1) per update.
Memory: Stack-only, zero allocation.
Ideal for 100–10kHz control loops on MCUs.
🧪 Testing

Includes tests for:

Step response stability and convergence.
Zero/negative dt handling.
Setpoint tracking and safety limit enforcement.

Run:

cargo test -- --nocapture
# ⚖️ License

GPL-2.0-or-later © 2026 Jorge Andre Castro.

Free to use, modify, and distribute. Any derivative works must also be GPL-2.0-or-later.
