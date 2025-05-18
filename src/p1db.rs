pub fn input_to_output_db(input_p1db: f64, gain_db: f64) -> f64 {
    input_p1db + (gain_db - 1.0)
}

pub fn output_to_input_db(output_p1db: f64, gain_db: f64) -> f64 {
    output_p1db - (gain_db - 1.0)
}

// /// Calculate the output P1dB of a cascade of stages.
// /// https://www.rfcafe.com/references/electrical/p1db.htm
//
// pub fn cascade_output_p1db_linear(
//     cumulative_output_p1db_linear: f64,
//     current_stage_output_p1db_linear: f64,
//     current_stage_gain_linear: f64,
// ) -> f64 {
//     1.0 / ((1.0 / cumulative_output_p1db_linear * current_stage_gain_linear)
//         + (1.0 / current_stage_output_p1db_linear))
// }
// pub fn cascade_output_p1db(
//     cumulative_output_p1db: f64,
//     current_stage_output_p1db: f64,
//     current_stage_gain: f64,
// ) -> f64 {
//     let cumulative_output_p1db_linear = crate::power::db_to_linear(cumulative_output_p1db);
//     let current_stage_output_linear = crate::power::db_to_linear(current_stage_output_p1db);
//     let current_stage_gain_linear = crate::power::db_to_linear(current_stage_gain);
//     let cascade_output_p1db_linear = cascade_output_p1db_linear(
//         cumulative_output_p1db_linear,
//         current_stage_output_linear,
//         current_stage_gain_linear,
//     );
//     crate::power::linear_to_db(cascade_output_p1db_linear)
// }

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
    // #[test]
    // fn cascade_output_p1db() {
    //     let cumulative_output_p1db: f64 = 34.0;

    //     let current_stage_output_p1db: f64 = 20.0;

    //     let current_stage_gain: f64 = 30.0;

    //     let cascade_output_p1db = crate::p1db::cascade_output_p1db(
    //         cumulative_output_p1db,
    //         current_stage_output_p1db,
    //         current_stage_gain,
    //     );
    //     assert_eq!(cascade_output_p1db, 16.0);
    // }
}
