# so2nostd

[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL%202.0%2B-blue.svg)](https://www.gnu.org/licenses/gpl-2.0)
[![No Std](https://img.shields.io/badge/no_std-compatible-green.svg)](https://docs.rs/so2nostd)
[![Maintenance](https://img.shields.io/badge/Maintenance-Actively--developed-brightgreen.svg)](https://github.com/jorgeandrecastro/so2nostd)

## Second-Order `no_std` Controller for Embedded Systems

**so2nostd** is a lightweight, high-performance `no_std` Rust crate implementing a discrete-time second-order (SO2) control system. Designed for embedded environments like MCUs (e.g., RP2040), it provides stable, physics-based dynamics using Euler integration. 

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
- 🔧 **Size-Optimized**: Compatible with `opt-level="z"` and LTO for tiny binary footprints.
- 🛡️ **Numerical Safety**: Robust handling of `dt <= 0` to prevent system divergence.
- 📈 **Physics-Based**: Models natural frequency (`ω_n`), damping ratio (`ζ`), and static gain.

## 🛠️ Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
so2nostd = { git = "[https://github.com/jorgeandrecastro/so2nostd.git](https://github.com/jorgeandrecastro/so2nostd.git)" }
For f32 support (Cortex-M optimization):Ini, TOMLso2nostd = { git = "[https://github.com/jorgeandrecastro/so2nostd.git](https://github.com/jorgeandrecastro/so2nostd.git)", features = ["f32"] }
🚀 QuickstartRustuse so2nostd::So2Controller;

fn main() {
    // w_n=10 rad/s, zeta=0.7 (underdamped), initial=0.0, gain=1.0
    let mut controller = So2Controller::new(10.0, 0.7, 0.0, 1.0);

    let dt = 0.01; // 10ms timestep
    let target = 1.0;

    // Simulate convergence
    for _ in 0..100 {
        let output = controller.update(target, dt);
        // output smoothly approaches 1.0 following SO2 dynamics
    }
}
📚 API ReferenceMethodSignatureDescriptionnewnew(w_n: Float, zeta: Float, initial: Float, gain: Float)Creates a new controller. w_n: rad/s, zeta: damping.updateupdate(input: Float, dt: Float) -> FloatAdvances system state. Safe for dt <= 0.set_targetset_target(target: Float)Updates the internal setpoint.resetreset(value: Float)Hard reset of internal states to value.Note: Float is an alias for f64 (default) or f32 (with feature).

⚡ Performance & OptimizationZero Allocation: No heap usage, strictly stack-based.Predictable CPU: Constant time O(1) update cycles.Binary Size: Minimal footprint when compiled with panic = "abort" and strip = true.🧪 TestingThe crate includes tests for:Step response stability and convergence.Zero/Negative delta time safety.Run tests with:Bashcargo test

⚖️ LicenseGPL-2.0-or-later © 2026 Jorge Andre Castro.Free to use, modify, and distribute. In accordance with the GPL, any derivative works or larger projects incorporating this code must also be released under the GPL-2.0-or-later.