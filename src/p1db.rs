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
}
