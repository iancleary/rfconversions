pub fn input_to_output_db(input_p1db: f64, gain_linear: f64) -> f64 {
    input_p1db + (gain_linear - 1)
}

pub fn output_to_input_db(output_p1db: f64, gain_linear: f64) -> f64 {
    output_p1db - (gain_linear - 1)
}

pub fn cascade_output_p1db_linear(cumulative_output_p1db_linear: f64, current_stage_output_p1dB: f64, current_stage_gain_linear: f64) -> f64 {
    1.0 / ((1/cumulative_output_p1db*current_stage_gain_linear) + (1.0 / gain_linear))
}
pub fn cascade_output_p1db_dB(cumulative_output_p1db: f64, current_stage_output_p1dB: f64, current_stage_gain: f64) -> f64 {
    let cumulative_output_p1db_linear = crate::power::db_to_linear(cumulative_output_p1db);
    let current_stage_output_linear = crate::power::db_to_linear(current_stage_output_p1dB);
    let current_stage_gain_linear = crate::power::db_to_linear(current_stage_gain);
    let cascade_output_p1db_linear = cascade_output_p1db_linear(cumulative_output_p1db_linear, current_stage_output_linear, current_stage_gain_linear);
    crate::power::linear_to_db(cascade_output_p1db_linear)
}


