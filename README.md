# rfconversions

[![Crates.io](https://img.shields.io/crates/v/rfconversions.svg)](https://crates.io/crates/rfconversions)

Common conversion equations for RF Engineering.

This guide walks through the library progressively — power, frequency, noise, and compression point conversions — with examples you can copy into your own code.

## 1. Power Conversions

Convert between watts and dBm, or between dB and linear scale.

```rust
use rfconversions::power;

// Watts ↔ dBm
let dbm = power::watts_to_dbm(1.0);    // 30.0 dBm
let watts = power::dbm_to_watts(30.0);  // 1.0 W

// Milliwatts ↔ dBm
let mw = power::dbm_to_milliwatts(0.0); // 1.0 mW
let dbm = power::milliwatts_to_dbm(1.0); // 0.0 dBm

// Watts ↔ dBW
let dbw = power::watts_to_dbw(1.0);     // 0.0 dBW
let watts = power::dbw_to_watts(0.0);   // 1.0 W

// Milliwatts ↔ dBW
let dbw = power::milliwatts_to_dbw(1000.0); // 0.0 dBW
let mw = power::dbw_to_milliwatts(0.0);     // 1000.0 mW

// dBm ↔ dBW
let dbw = power::dbm_to_dbw(30.0);      // 0.0 dBW
let dbm = power::dbw_to_dbm(0.0);       // 30.0 dBm

// dB ↔ Linear
let linear = power::db_to_linear(30.0); // 1000.0
let db = power::linear_to_db(1000.0);   // 30.0 dB
```

## 2. Frequency Conversions

Scale between Hz, kHz, MHz, GHz, and THz, or convert frequency to wavelength.

```rust
use rfconversions::frequency;

// Unit scaling
let ghz = frequency::mhz_to_ghz(2400.0);  // 2.4 GHz
let hz  = frequency::ghz_to_hz(1.0);       // 1_000_000_000.0 Hz
let mhz = frequency::khz_to_mhz(1500.0);  // 1.5 MHz
let thz = frequency::ghz_to_thz(1000.0);   // 1.0 THz

// Frequency → Wavelength (meters, in vacuum)
let wavelength = frequency::frequency_to_wavelength(1.0e9); // 0.299792458 m
```

## 3. Noise

Convert between noise figure (dB), noise factor (linear), and noise temperature (K). Compute noise power from bandwidth.

```rust
use rfconversions::noise;

// Noise factor (linear) ↔ Noise figure (dB)
let nf_db = noise::noise_figure_from_noise_factor(2.0);       // ~3.01 dB
let nf_linear = noise::noise_factor_from_noise_figure(3.010299956639812); // 2.0

// Noise temperature ↔ Noise factor
let temp = noise::noise_temperature_from_noise_factor(2.0);   // 290.0 K
let factor = noise::noise_factor_from_noise_temperature(290.0); // 2.0

// Noise temperature ↔ Noise figure
let temp2 = noise::noise_temperature_from_noise_figure(6.0);  // ~864.51 K
let nf_db2 = noise::noise_figure_from_noise_temperature(290.0); // ~3.01 dB

// Noise power (W) from temperature and bandwidth
let noise_power = noise::noise_power_from_bandwidth(290.0, 100.0e6); // kTB in watts
```

## 4. P1dB Compression Point

Convert between input and output 1 dB compression points.

The relationship is: `OP1dB = IP1dB + (Gain - 1)` (all in dB).

```rust
use rfconversions::p1db;

let output_p1db = p1db::input_to_output_db(5.0, 30.0);  // 34.0 dBm
let input_p1db  = p1db::output_to_input_db(34.0, 30.0);  // 5.0 dBm
```

## 5. Friis Cascade (Noise)

Cascade noise figure, noise factor, or noise temperature through a chain of stages using the Friis formula.

```rust
use rfconversions::noise;

// LNA (NF=0.5dB, G=20dB) → Cable (NF=1dB, G=-1dB) → Mixer (NF=8dB, G=-7dB)
let stages = vec![(0.5, 20.0), (1.0, -1.0), (8.0, -7.0)];
let nf_total = noise::cascade_noise_figure(&stages);
assert!((nf_total - 0.754).abs() < 0.01); // LNA dominates

// Same chain in linear domain: stages are (noise_factor, gain_linear)
let linear_stages = vec![
    (1.1220, 100.0),   // LNA
    (1.2589, 0.7943),  // Cable
    (6.3096, 0.1995),  // Mixer
];
let f_total = noise::cascade_noise_factor(&linear_stages);
assert!((f_total - 1.1914).abs() < 0.001);

// Noise temperature cascade: stages are (Te_kelvin, gain_linear)
let temp_stages = vec![(35.0, 100.0), (75.0, 0.794)];
let t_total = noise::cascade_noise_temperature(&temp_stages);
assert!((t_total - 35.75).abs() < 0.01);
```

## 6. System-Level Helpers

G/T (figure of merit) and noise power spectral density N₀.

```rust
use rfconversions::noise;

// G/T: 40 dBi antenna, 200 K system noise → 17.0 dB/K
let got = noise::g_over_t(40.0, 200.0);
assert!((got - 16.99).abs() < 0.01);

// N₀: thermal noise floor at 290 K → -174 dBm/Hz
let n0 = noise::noise_density_dbm_per_hz(290.0);
assert!((n0 - (-174.0)).abs() < 0.1);
```

## 7. Constants

Physical constants used internally, available for your own calculations.

```rust
use rfconversions::constants;

let c = constants::SPEED_OF_LIGHT;  // 299_792_458.0 m/s
let k = constants::BOLTZMANN;       // 1.380649e-23 J/K
let t0 = constants::T0;             // 290.0 K (standard reference)
```

## API Summary

| Module | Function | Description |
|---|---|---|
| `power` | `watts_to_dbm(f64) → f64` | Watts to dBm |
| `power` | `dbm_to_watts(f64) → f64` | dBm to Watts |
| `power` | `dbm_to_milliwatts(f64) → f64` | dBm to milliwatts |
| `power` | `milliwatts_to_dbm(f64) → f64` | Milliwatts to dBm |
| `power` | `watts_to_dbw(f64) → f64` | Watts to dBW |
| `power` | `dbw_to_watts(f64) → f64` | dBW to Watts |
| `power` | `milliwatts_to_dbw(f64) → f64` | Milliwatts to dBW |
| `power` | `dbw_to_milliwatts(f64) → f64` | dBW to milliwatts |
| `power` | `dbm_to_dbw(f64) → f64` | dBm to dBW |
| `power` | `dbw_to_dbm(f64) → f64` | dBW to dBm |
| `power` | `db_to_linear(f64) → f64` | dB to linear ratio |
| `power` | `linear_to_db(f64) → f64` | Linear ratio to dB |
| `frequency` | `frequency_to_wavelength(f64) → f64` | Frequency (Hz) to wavelength (m) |
| `frequency` | `wavelength_to_frequency(f64) → f64` | Wavelength (m) to frequency (Hz) |
| `frequency` | `hz_to_khz`, `hz_to_mhz`, `hz_to_ghz`, `hz_to_thz` | Hz scaling up |
| `frequency` | `khz_to_hz`, `khz_to_mhz`, `khz_to_ghz`, `khz_to_thz` | kHz scaling |
| `frequency` | `mhz_to_hz`, `mhz_to_khz`, `mhz_to_ghz`, `mhz_to_thz` | MHz scaling |
| `frequency` | `ghz_to_hz`, `ghz_to_khz`, `ghz_to_mhz`, `ghz_to_thz` | GHz scaling |
| `frequency` | `thz_to_hz`, `thz_to_khz`, `thz_to_mhz`, `thz_to_ghz` | THz scaling |
| `noise` | `noise_figure_from_noise_factor(f64) → f64` | Factor → Figure (dB) |
| `noise` | `noise_factor_from_noise_figure(f64) → f64` | Figure → Factor |
| `noise` | `noise_temperature_from_noise_factor(f64) → f64` | Factor → Temperature (K) |
| `noise` | `noise_temperature_from_noise_figure(f64) → f64` | Figure → Temperature (K) |
| `noise` | `noise_factor_from_noise_temperature(f64) → f64` | Temperature → Factor |
| `noise` | `noise_figure_from_noise_temperature(f64) → f64` | Temperature → Figure (dB) |
| `noise` | `noise_power_from_bandwidth(f64, f64) → f64` | kTB noise power (W) |
| `noise` | `cascade_noise_factor(&[(f64, f64)]) → f64` | Friis cascade (linear) |
| `noise` | `cascade_noise_figure(&[(f64, f64)]) → f64` | Friis cascade (dB) |
| `noise` | `cascade_noise_temperature(&[(f64, f64)]) → f64` | Friis cascade (Kelvin) |
| `noise` | `g_over_t(f64, f64) → f64` | G/T figure of merit (dB/K) |
| `noise` | `noise_density_dbm_per_hz(f64) → f64` | N₀ noise density (dBm/Hz) |
| `p1db` | `input_to_output_db(f64, f64) → f64` | IP1dB + Gain → OP1dB |
| `p1db` | `output_to_input_db(f64, f64) → f64` | OP1dB − Gain → IP1dB |
| `p1db` | `cascade_output_p1db(f64, f64, f64) → f64` | Cascade OP1dB (dB) |
| `p1db` | `cascade_output_p1db_linear(f64, f64, f64) → f64` | Cascade OP1dB (linear) |
| `constants` | `SPEED_OF_LIGHT` | 299 792 458 m/s |
| `constants` | `BOLTZMANN` | 1.380649e-23 J/K |
| `constants` | `T0` | 290 K reference temperature |

## License

MIT
