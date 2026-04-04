# so2nostd

[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL%202.0-blue.svg)](https://www.gnu.org/licenses/gpl-2.0)
[![No Std](https://img.shields.io/badge/no_std-compatible-green.svg)](https://docs.rs/so2nostd)
[![Maintenance](https://img.shields.io/badge/Maintenance-Actively--developed-brightgreen.svg)](https://github.com/jorgeandrecastro/so2nostd)

## Second-Order `no_std` Controller for Embedded Systems

**so2nostd** is a lightweight, high-performance `no_std` Rust crate implementing a discrete-time second-order (SO2) control system. Designed for embedded environments like MCUs (e.g., RP2040), it provides stable, physics-based dynamics using Euler integration. 

GPL-2.0-or-later licensed to ensure community protection against privatization. Optimized for minimal footprint and maximal reliability.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Quickstart](#quickstart)
- [Examples](#examples)
- [API Reference](#api-reference)
- [Performance & Optimization](#performance--optimization)
- [Testing](#testing)
- [License](#license)
- [Contributing](#contributing)

## 🚀 Features
- ✅ **Pure `no_std`**: Zero standard library dependencies, perfect for bare-metal/RTOS.
- ⚡ **Flexible Floating-Point**: `f64` (default, precision) or `f32` feature for memory-constrained devices.
- 🔧 **Size-Optimized Builds**: Release profile with `opt-level=\"z\"`, LTO, stripping for tiny binaries.
- 🛡️ **Robust & Safe**: Handles `dt <= 0`, numerical stability, real physics (acceleration from forces).
- 📈 **Second-Order Dynamics**: Natural frequency (`ω_n`), damping (`ζ`), gain, setpoint tracking.

## 🛠️ Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
so2nostd = { git = "https://github.com/jorgeandrecastro/so2nostd.git" }
```

**f32 for embedded (e.g., Cortex-M):**
```toml
so2nostd = { git = "https://github.com/jorgeandrecastro/so2nostd.git", features = ["f32"] }
```

Build with optimizations:
```bash
cargo build --release
```

## 🚀 Quickstart

```rust
use so2nostd::So2Controller;

fn main() {
    // ω_n=10 rad/s, ζ=0.7 (underdamped), initial=0, gain=1.0
    let mut controller = So2Controller::new(10.0, 0.7, 0.0, 1.0);

    let dt = 0.01; // 10ms timestep
    let target = 1.0;

    // Simulate convergence (outputs ~0.0 → 1.0 over time)
    for step in 0..20 {
        let output = controller.update(target, dt);
        println!("Step {}: output = {:.4}", step, output);
        // e.g., Step 0: 0.0950, ..., Step 19: ~0.99
    }
}
```

The system smoothly converges to the setpoint following SO2 dynamics without overshoot/divergence.

## 📖 Examples

### 1. Dynamic Setpoint Change (e.g., Motor Position Control)
```rust
let mut controller = So2Controller::new(20.0, 0.8, 0.0, 1.0);
controller.set_target(5.0); // Ramp to 5.0
let output = controller.update(controller.setpoint, 0.005);
controller.reset(0.0); // Reset for next cycle
```

### 2. Embedded Loop (no_std + RTOS)
Suitable for PID-like control in motor/servos, filters, etc.

## 📚 API Reference

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `So2Controller::new(w_n: Float, zeta: Float, initial_value: Float, gain: Float) -> Self` | Creates controller. `w_n`: rad/s, `zeta`: damping (0.7 typical). |
| `update` | `&amp;mut self.update(input: Float, dt: Float) -> Float` | Updates state, returns new output. Safe for `dt <= 0`. |
| `set_target` | `&amp;mut self.set_target(target: Float)` | Updates setpoint. |
| `reset` | `&amp;mut self.reset(value: Float)` | Resets states to `value`. |

**Type**: `Float = f64` (or `f32` with feature).

Public fields: `w_n`, `zeta` (inspectable/tunable).

## ⚡ Performance & Optimization

- **Binary Size**: ~1-2KB (release, varies by Float).
- **CPU**: O(1) per update, no allocs/loops.
- **Profile**: `cargo build --release` auto-applies `opt-level="z"`, LTO, `panic=abort`.
- Ideal for 100-10kHz control loops on MCUs.

## 🧪 Testing

Includes unit tests for:
- Step response stability/convergence.
- Zero `dt` handling.

Run: `cargo test`

```bash
cargo test -- --nocapture
```

## ⚖️ License

GPL-2.0-or-later © 2026 Jorge Andre Castro.

Free to use/modify/distribute, but derivatives must remain open-source GPL.

[Full LICENSE](LICENSE)

## 🤝 Contributing

1. Fork &amp; PR to `main`.
2. Follow Rustfmt: `cargo fmt`.
3. Add tests for new features.
4. Respect GPL: No proprietary forks.

[Issues](https://github.com/jorgeandrecastro/so2nostd/issues) | [Docs](https://docs.rs/so2nostd)
