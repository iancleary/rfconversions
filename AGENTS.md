# AGENTS.md - rfconversions

Rust crate for RF engineering conversions: power, frequency/wavelength, noise
figure/factor/temperature, P1dB, constants, and system-level helpers.

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
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
cargo doc --open
just cut-release --dry-run --version <semver>
```

## Releases

Maintain the deterministic release workflow with `create-release-process`.
Execute ordinary releases with `cut-release` via `just cut-release`; see
`docs/release.md` for the repo-local contract. The runner requires an explicit
SemVer `--version`, supports read-only version queries, and creates the GitHub
release as the final public step of a real release.

## Notes

- Keep changes minimal and aligned to the crate's RF conversion purpose.
- Run `cargo fmt -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` for behavior changes.
- Claude Code guidance lives in `CLAUDE.md`; keep both files consistent when changing repo workflows.
