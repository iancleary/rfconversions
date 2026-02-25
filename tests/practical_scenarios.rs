//! Practical RF engineering scenario tests.
//!
//! These integration tests verify rfconversions functions against
//! real-world RF values commonly encountered in satellite and
//! wireless system design.

use rfconversions::frequency::*;
use rfconversions::noise::*;
use rfconversions::p1db::*;
use rfconversions::power::*;

// ── Power conversions at practical levels ────────────────────────

#[test]
fn typical_transmitter_power_levels() {
    // 100W SSPA → 50 dBm → 20 dBW
    let dbm = watts_to_dbm(100.0);
    assert!((dbm - 50.0).abs() < 1e-10);
    assert!((dbm_to_dbw(dbm) - 20.0).abs() < 1e-10);

    // 1W → 30 dBm → 0 dBW
    assert!((watts_to_dbm(1.0) - 30.0).abs() < 1e-10);
    assert!((watts_to_dbw(1.0) - 0.0).abs() < 1e-10);

    // 10 mW → 10 dBm
    assert!((milliwatts_to_dbm(10.0) - 10.0).abs() < 1e-10);
}

#[test]
fn receiver_sensitivity_levels() {
    // Typical GPS receiver sensitivity: -130 dBm
    let watts = dbm_to_watts(-130.0);
    assert!((watts - 1e-16).abs() < 1e-26);

    // Roundtrip at very low power
    let roundtrip = watts_to_dbm(dbm_to_watts(-130.0));
    assert!((roundtrip - (-130.0)).abs() < 1e-10);
}

#[test]
fn dbm_dbw_roundtrip_across_range() {
    // Test roundtrip conversion across a wide dynamic range
    for dbm_val in [-174, -130, -100, -50, 0, 10, 30, 43, 50, 60] {
        let dbm = dbm_val as f64;
        let dbw = dbm_to_dbw(dbm);
        let back = dbw_to_dbm(dbw);
        assert!(
            (back - dbm).abs() < 1e-10,
            "Roundtrip failed at {dbm} dBm"
        );
    }
}

#[test]
fn watts_milliwatts_roundtrip() {
    for &mw in &[0.001, 0.01, 1.0, 100.0, 1000.0, 100_000.0] {
        let dbm = milliwatts_to_dbm(mw);
        let back = dbm_to_milliwatts(dbm);
        assert!(
            (back - mw).abs() / mw < 1e-12,
            "Roundtrip failed at {mw} mW"
        );
    }
}

// ── Noise figure cascading ──────────────────────────────────────

#[test]
fn lna_noise_temperature_from_nf() {
    // Typical LNA: NF = 0.5 dB → Te ≈ 35.4 K
    let te = noise_temperature_from_noise_figure(0.5);
    assert!((te - 35.381).abs() < 0.1);
}

#[test]
fn noise_figure_roundtrip() {
    for &nf_db in &[0.5, 1.0, 3.0, 6.0, 10.0] {
        let factor = noise_factor_from_noise_figure(nf_db);
        let temp = noise_temperature_from_noise_factor(factor);
        let factor_back = noise_factor_from_noise_temperature(temp);
        let nf_back = noise_figure_from_noise_factor(factor_back);
        assert!(
            (nf_back - nf_db).abs() < 1e-10,
            "Noise figure roundtrip failed at {nf_db} dB"
        );
    }
}

#[test]
fn thermal_noise_floor() {
    // kTB noise power at 290K, 1 Hz bandwidth = -174 dBm/Hz
    let power_w = noise_power_from_bandwidth(290.0, 1.0);
    let power_dbm = watts_to_dbm(power_w);
    assert!(
        (power_dbm - (-174.0)).abs() < 0.1,
        "Thermal noise floor was {power_dbm} dBm, expected ~-174 dBm"
    );
}

#[test]
fn noise_power_36mhz_transponder() {
    // 36 MHz transponder bandwidth at 290K system temp
    let bw = 36.0e6;
    let power_w = noise_power_from_bandwidth(290.0, bw);
    let power_dbm = watts_to_dbm(power_w);
    // Expected: -174 + 10*log10(36e6) ≈ -174 + 75.56 = -98.44 dBm
    assert!(
        (power_dbm - (-98.44)).abs() < 0.1,
        "36 MHz noise power was {power_dbm} dBm"
    );
}

// ── Frequency / wavelength at common bands ──────────────────────

#[test]
fn satellite_band_wavelengths() {
    // Ka-band downlink: 20 GHz → 15 mm
    let wl = frequency_to_wavelength(ghz_to_hz(20.0));
    assert!((wl * 1000.0 - 14.99).abs() < 0.01);

    // Ku-band downlink: 12 GHz → 25 mm
    let wl = frequency_to_wavelength(ghz_to_hz(12.0));
    assert!((wl * 1000.0 - 24.98).abs() < 0.02);

    // X-band: 10 GHz → 30 mm
    let wl = frequency_to_wavelength(ghz_to_hz(10.0));
    assert!((wl * 1000.0 - 29.98).abs() < 0.01);
}

#[test]
fn frequency_unit_chain_roundtrip() {
    // 27.5 GHz through all units and back
    let ghz = 27.5;
    let thz = ghz_to_thz(ghz);
    let mhz = thz_to_mhz(thz);
    let khz = mhz_to_khz(mhz);
    let hz = khz_to_hz(khz);
    let back = hz_to_ghz(hz);
    assert!(
        (back - ghz).abs() < 1e-10,
        "Frequency chain roundtrip: got {back}, expected {ghz}"
    );
}

// ── P1dB cascade scenarios ──────────────────────────────────────

#[test]
fn three_stage_receive_chain_p1db() {
    // LNA (OP1dB=15dBm, G=20dB) → Filter (OP1dB=40dBm, G=-3dB) → Mixer (OP1dB=10dBm, G=-7dB)
    // Stage 1 sets initial cumulative OP1dB
    let cum = 15.0_f64;

    // Stage 2: filter (high OP1dB, negative gain = loss)
    let cum = cascade_output_p1db(cum, 40.0, -3.0);

    // Stage 3: mixer
    let cum = cascade_output_p1db(cum, 10.0, -7.0);

    // The mixer's OP1dB should dominate since it has low OP1dB
    // and preceding gain is reduced by losses. Result should be
    // somewhere between the mixer's OP1dB and the cascade-degraded value.
    assert!(
        cum < 10.0,
        "Cascade OP1dB should be less than mixer's OP1dB, got {cum}"
    );
    assert!(cum > -10.0, "Cascade OP1dB unreasonably low: {cum}");
}

#[test]
fn p1db_input_output_roundtrip_negative_gain() {
    // Attenuator: 10 dB loss (gain = -10 dB)
    let ip1db = 20.0;
    let gain = -10.0;
    let op1db = input_to_output_db(ip1db, gain);
    let back = output_to_input_db(op1db, gain);
    assert!((back - ip1db).abs() < 1e-10);
}

// ── Cross-module practical calculations ─────────────────────────

#[test]
fn gt_ratio_components() {
    // G/T calculation building blocks:
    // Antenna gain = 40 dBi, System noise temp = 200K
    // G/T = 40 - 10*log10(200) = 40 - 23.01 = 16.99 dB/K
    let gain_dbi = 40.0;
    let tsys = 200.0;
    let tsys_db = linear_to_db(tsys); // 10*log10(200)
    let g_over_t = gain_dbi - tsys_db;
    assert!(
        (g_over_t - 16.99).abs() < 0.01,
        "G/T was {g_over_t}, expected ~16.99"
    );
}

#[test]
fn eirp_calculation() {
    // EIRP = Tx Power (dBW) + Antenna Gain (dBi)
    // 10W transmitter + 35 dBi dish
    let tx_dbw = watts_to_dbw(10.0);
    let gain_dbi = 35.0;
    let eirp = tx_dbw + gain_dbi;
    assert!(
        (eirp - 45.0).abs() < 0.01,
        "EIRP was {eirp}, expected 45.0 dBW"
    );
}
