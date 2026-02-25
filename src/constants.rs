/// Speed of light in vacuum, in meters per second.
///
/// # Examples
///
/// ```
/// use rfconversions::constants::SPEED_OF_LIGHT;
/// assert_eq!(SPEED_OF_LIGHT, 299_792_458.0);
/// ```
pub const SPEED_OF_LIGHT: f64 = 299792458.0;

#[cfg(test)]
mod tests {

    #[test]
    fn speed_of_light() {
        use super::SPEED_OF_LIGHT;

        assert_eq!(299792458.0, SPEED_OF_LIGHT);
    }
}
