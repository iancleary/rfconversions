/// Convert input P1dB to output P1dB (both in dB/dBm).
///
/// Output P1dB = Input P1dB + Gain - 1 dB.
///
/// # Examples
///
/// ```
/// use rfconversions::p1db::input_to_output_db;
/// assert_eq!(input_to_output_db(5.0, 30.0), 34.0);
/// ```
#[doc(alias = "IP1dB")]
#[doc(alias = "OP1dB")]
#[must_use]
pub fn input_to_output_db(input_p1db: f64, gain_db: f64) -> f64 {
    input_p1db + (gain_db - 1.0)
}

/// Convert output P1dB to input P1dB (both in dB/dBm).
///
/// Input P1dB = Output P1dB - Gain + 1 dB.
///
/// # Examples
///
/// ```
/// use rfconversions::p1db::output_to_input_db;
/// assert_eq!(output_to_input_db(34.0, 30.0), 5.0);
/// ```
#[doc(alias = "IP1dB")]
#[doc(alias = "OP1dB")]
#[must_use]
pub fn output_to_input_db(output_p1db: f64, gain_db: f64) -> f64 {
    output_p1db - (gain_db - 1.0)
}

/// Calculate the output P1dB of a cascade of stages (linear domain).
///
/// Reference: <https://www.rfcafe.com/references/electrical/p1db.htm>
///
/// # Examples
///
/// ```
/// use rfconversions::p1db::cascade_output_p1db_linear;
/// // 1 / (1/100 * 2 + 1/50) = 1 / (0.02 + 0.02) = 25.0
/// let result = cascade_output_p1db_linear(100.0, 50.0, 2.0);
/// assert_eq!(result, 25.0);
/// ```
#[doc(alias = "OP1dB")]
#[must_use]
pub fn cascade_output_p1db_linear(
    cumulative_output_p1db_linear: f64,
    current_stage_output_p1db_linear: f64,
    current_stage_gain_linear: f64,
) -> f64 {
    1.0 / ((1.0 / cumulative_output_p1db_linear * current_stage_gain_linear)
        + (1.0 / current_stage_output_p1db_linear))
}

/// Calculate the output P1dB of a cascade of stages (dB domain).
///
/// Converts to linear, applies the cascade formula, then converts back.
///
/// # Examples
///
/// ```
/// use rfconversions::p1db::cascade_output_p1db;
/// let result = cascade_output_p1db(34.0, 20.0, 30.0);
/// let rounded = (result * 1e5).round() / 1e5;
/// assert_eq!(rounded, 3.89226);
/// ```
#[doc(alias = "OP1dB")]
#[doc(alias = "P1dB")]
#[must_use]
pub fn cascade_output_p1db(
    cumulative_output_p1db: f64,
    current_stage_output_p1db: f64,
    current_stage_gain: f64,
) -> f64 {
    let cumulative_output_p1db_linear = crate::power::db_to_linear(cumulative_output_p1db);
    let current_stage_output_linear = crate::power::db_to_linear(current_stage_output_p1db);
    let current_stage_gain_linear = crate::power::db_to_linear(current_stage_gain);
    let cascade_output_p1db_linear = cascade_output_p1db_linear(
        cumulative_output_p1db_linear,
        current_stage_output_linear,
        current_stage_gain_linear,
    );
    crate::power::linear_to_db(cascade_output_p1db_linear)
}

#[cfg(test)]
mod tests {

    #[test]
    fn input_to_output_p1db() {
        let input_p1db: f64 = 5.0;

        let gain_db: f64 = 30.0;

        let output_p1db = crate::p1db::input_to_output_db(input_p1db, gain_db);
        assert_eq!(output_p1db, 34.0);
    }

    #[test]
    fn output_to_input_p1db() {
        let output_p1db: f64 = 34.0;

        let gain_db: f64 = 30.0;

        let input_p1db = crate::p1db::output_to_input_db(output_p1db, gain_db);
        assert_eq!(input_p1db, 5.0);
    }

    // https://www.rfcafe.com/references/electrical/p1db.htm
    #[test]
    fn cascade_output_p1db() {
        let cumulative_output_p1db: f64 = 34.0;

        let current_stage_output_p1db: f64 = 20.0;

        let current_stage_gain: f64 = 30.0;

        let cascade_output_p1db = crate::p1db::cascade_output_p1db(
            cumulative_output_p1db,
            current_stage_output_p1db,
            current_stage_gain,
        );
        let rounded = (cascade_output_p1db * 1e5).round() / 1e5;
        assert_eq!(rounded, 3.89226);
    }

    #[test]
    fn roundtrip_input_to_output_to_input() {
        let input_p1db: f64 = -10.0;
        let gain_db: f64 = 25.0;

        let output_p1db = crate::p1db::input_to_output_db(input_p1db, gain_db);
        let result = crate::p1db::output_to_input_db(output_p1db, gain_db);

        assert_eq!(input_p1db, result);
    }

    #[test]
    fn input_to_output_negative_gain_attenuator() {
        // Attenuator: gain = -10 dB, input P1dB = 20 dBm
        // Output P1dB = 20 + (-10 - 1) = 9 dBm
        let result = crate::p1db::input_to_output_db(20.0, -10.0);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn input_to_output_zero_gain() {
        // Unity gain device: output P1dB = input P1dB - 1
        let result = crate::p1db::input_to_output_db(10.0, 0.0);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn output_to_input_negative_gain() {
        // Attenuator: output P1dB = 9 dBm, gain = -10 dB
        // Input P1dB = 9 - (-10 - 1) = 9 + 11 = 20
        let result = crate::p1db::output_to_input_db(9.0, -10.0);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn cascade_three_stage_amplifier_chain() {
        // Three-stage cascade: apply formula iteratively
        // Stage 1: OP1dB = 30 dBm, Gain = 20 dB
        // Stage 2: OP1dB = 25 dBm, Gain = 15 dB
        // Stage 3: OP1dB = 35 dBm, Gain = 10 dB
        let stages: Vec<(f64, f64)> = vec![(30.0, 20.0), (25.0, 15.0), (35.0, 10.0)];

        let mut cumulative = stages[0].0; // first stage OP1dB
        for &(op1db, gain) in &stages[1..] {
            cumulative = crate::p1db::cascade_output_p1db(cumulative, op1db, gain);
        }

        // Result should be dominated by the weakest stage referred to output
        // Just verify it's a reasonable value less than any single stage OP1dB
        assert!(cumulative < 25.0, "cascade should be limited by weakest stage");
        assert!(cumulative > 0.0, "cascade should be positive");
    }

    #[test]
    fn cascade_identical_stages() {
        // Two identical stages: OP1dB = 20 dBm, Gain = 10 dB each
        let result = crate::p1db::cascade_output_p1db(20.0, 20.0, 10.0);
        // cascade < single stage OP1dB since first stage compresses at output
        assert!(result < 20.0);
    }

    #[test]
    fn cascade_linear_known_value() {
        // Simple case: both stages OP1dB = 100 mW linear, gain = 1 (0 dB)
        // 1 / (1/100 * 1 + 1/100) = 1 / 0.02 = 50
        let result = crate::p1db::cascade_output_p1db_linear(100.0, 100.0, 1.0);
        assert_eq!(result, 50.0);
    }

    #[test]
    fn cascade_high_gain_stage_dominates() {
        // High gain second stage makes first stage's P1dB dominate
        // cumulative OP1dB = 20 dBm, next stage OP1dB = 40 dBm, gain = 40 dB
        let result = crate::p1db::cascade_output_p1db(20.0, 40.0, 40.0);
        // With 40 dB gain, first stage P1dB referred to output = 20 - 40 = -20 dBm equivalent
        // So cascade should be well below the first stage
        assert!(result < 20.0);
    }

    #[test]
    fn roundtrip_multiple_gains() {
        // Roundtrip for several gain values
        for gain in [-20.0, -5.0, 0.0, 10.0, 30.0, 50.0] {
            let ip1db = 5.0;
            let op1db = crate::p1db::input_to_output_db(ip1db, gain);
            let back = crate::p1db::output_to_input_db(op1db, gain);
            assert!(
                (back - ip1db).abs() < 1e-10,
                "roundtrip failed for gain={gain}"
            );
        }
    }
}
