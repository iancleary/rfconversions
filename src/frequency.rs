pub fn frequency_to_wavelength(frequency: f64) -> f64 {
    crate::constants::SPEED_OF_LIGHT / frequency
}

pub fn thz_to_hz(thz: f64) -> f64 {
    thz * 1e12
}

pub fn hz_to_thz(hz: f64) -> f64 {
    hz / 1e12
}

pub fn ghz_to_hz(ghz: f64) -> f64 {
    ghz * 1e9
}

pub fn hz_to_ghz(hz: f64) -> f64 {
    hz / 1e9
}

pub fn mhz_to_hz(mhz: f64) -> f64 {
    mhz * 1e6
}

pub fn hz_to_mhz(hz: f64) -> f64 {
    hz / 1e6
}

pub fn khz_to_hz(khz: f64) -> f64 {
    khz * 1e3
}

pub fn hz_to_khz(hz: f64) -> f64 {
    hz / 1e3
}

#[cfg(test)]
mod tests {
    #[test]
    fn frequency_to_wavelength_one_gigahertz() {
        let base: f64 = 10.0;
        let frequency: f64 = 1.0 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);

        assert_eq!(0.299792458, wavelength);
    }

    #[test]
    fn frequency_to_wavelength_twenty_seven_point_five_gigahertz() {
        let base: f64 = 10.0;
        let frequency: f64 = 27.5 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);

        assert_eq!(0.010901543927272727, wavelength);
    }

    #[test]
    fn frequency_to_wavelength_thirty_gigahertz() {
        let base: f64 = 10.0;
        let frequency: f64 = 30.0 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);

        assert_eq!(0.009993081933333333, wavelength);
    }

    #[test]
    fn test_frequency_conversions_hz_khz_mhz_ghz_thz() {
        let thz = 1.0;
        let ghz = 1000.0;
        let mhz = 1_000_000.0;
        let khz = 1_000_000_000.0;
        let hz = 1_000_000_000_000.0;
        assert_eq!(super::thz_to_hz(thz), hz);
        assert_eq!(super::hz_to_thz(hz), thz);
        assert_eq!(super::ghz_to_hz(ghz), hz);
        assert_eq!(super::hz_to_ghz(hz), ghz);
        assert_eq!(super::mhz_to_hz(mhz), hz);
        assert_eq!(super::hz_to_mhz(hz), mhz);
        assert_eq!(super::khz_to_hz(khz), hz);
        assert_eq!(super::hz_to_khz(hz), khz);
    }
}
