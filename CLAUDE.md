# CLAUDE.md — rfconversions

## Overview

Rust crate providing common RF engineering unit conversions: power (watts ↔ dBm, dB ↔ linear), frequency scaling (Hz/kHz/MHz/GHz/THz + wavelength), noise (NF ↔ noise factor ↔ noise temperature), P1dB compression point conversions, physical constants, and system-level helpers (G/T, N₀). Published on crates.io (v0.7.3).

## Agent Usage

Use `rfconversions` as the lowest-level math crate when a task is about RF
units or scalar conversions, not system simulation. Prefer it before hand-writing
`10.0_f64.powf(x / 10.0)`, dBm/watt conversions, kTB noise power, noise
figure/factor/temperature conversions, wavelength math, or IP1dB/OP1dB
translation.

Common handoff pattern:

- use `rfconversions` for unit normalization and scalar RF formulas
- use `gainlineup` when the question has ordered RF blocks or cascade noise
- use `touchstone` when the input is S-parameter `.sNp` data
- use `linkbudget` when the question is an end-to-end radio link, margin, BER,
  orbit, Doppler, PFD, or modulation problem

Keep dB quantities in dB only for additions/subtractions. Convert through
`power::db_to_linear` or the noise helpers before multiplying ratios, averaging
linear quantities, or applying Friis-style equations. Frequencies passed to
wavelength helpers are in Hz and returned wavelengths are in meters.

## Commands

```bash
cargo test                        # Run all 78 tests (v0.7.3)
cargo clippy -- -D warnings       # Lint
cargo fmt -- --check              # Format check
cargo doc --open                  # Generate and view API docs
```

## Releases

Maintain the release workflow with the Forge-managed `create-release-process` skill. Execute ordinary releases with the repo-local `cut-release` flow documented in `docs/release.md`:

```bash
just cut-release --dry-run --version <semver>
just cut-release --version <semver>
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
