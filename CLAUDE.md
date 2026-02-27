# CLAUDE.md — rfconversions

## Overview

Rust crate providing common RF engineering unit conversions: power (watts ↔ dBm, dB ↔ linear), frequency scaling (Hz/kHz/MHz/GHz/THz + wavelength), noise (NF ↔ noise factor ↔ noise temperature), P1dB compression point conversions, physical constants, and system-level helpers (G/T, N₀). Published on crates.io (v0.7.2).

## Commands

```bash
cargo test                        # Run all 69 tests
cargo clippy -- -D warnings       # Lint
cargo fmt -- --check              # Format check
cargo doc --open                  # Generate and view API docs
```

## Module Map

| Module | File | Description |
|--------|------|-------------|
| `power` | `src/power.rs` | `watts_to_dbm`, `dbm_to_watts`, `db_to_linear`, `linear_to_db` |
| `frequency` | `src/frequency.rs` | Hz/kHz/MHz/GHz/THz scaling + `frequency_to_wavelength` |
| `noise` | `src/noise.rs` | NF ↔ noise factor ↔ noise temperature, `noise_power_from_bandwidth` |
| `p1db` | `src/p1db.rs` | `input_to_output_db`, `output_to_input_db` (IP1dB ↔ OP1dB) |
| `constants` | `src/constants.rs` | `SPEED_OF_LIGHT`, `BOLTZMANN`, `T0`, and other physical constants |

## Where to Look

- **README.md** — Complete API reference table with every function signature
- **src/lib.rs** — Module re-exports (5 public modules)
- Each module file contains the functions and their tests
- This is a leaf dependency used by `touchstone`, `gainlineup`, and `linkbudget`
