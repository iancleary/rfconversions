pub fn frequency_to_wavelength(frequency: f64) -> f64 {
    crate::constants::SPEED_OF_LIGHT / frequency
}

pub fn wavelength_to_frequency(wavelength: f64) -> f64 {
    crate::constants::SPEED_OF_LIGHT / wavelength
}

pub fn thz_to_ghz(thz: f64) -> f64 {
    thz * 1e3
}

pub fn thz_to_mhz(thz: f64) -> f64 {
    thz * 1e6
}

pub fn thz_to_khz(thz: f64) -> f64 {
    thz * 1e9
}

pub fn thz_to_hz(thz: f64) -> f64 {
    thz * 1e12
}

pub fn ghz_to_thz(ghz: f64) -> f64 {
    ghz / 1e3
}

pub fn ghz_to_mhz(ghz: f64) -> f64 {
    ghz * 1e3
}

pub fn ghz_to_khz(ghz: f64) -> f64 {
    ghz * 1e6
}

pub fn ghz_to_hz(ghz: f64) -> f64 {
    ghz * 1e9
}

pub fn mhz_to_thz(mhz: f64) -> f64 {
    mhz / 1e6
}

pub fn mhz_to_ghz(mhz: f64) -> f64 {
    mhz / 1e3
}

pub fn mhz_to_khz(mhz: f64) -> f64 {
    mhz * 1e3
}

pub fn mhz_to_hz(mhz: f64) -> f64 {
    mhz * 1e6
}

pub fn khz_to_thz(khz: f64) -> f64 {
    khz / 1e9
}

pub fn khz_to_ghz(khz: f64) -> f64 {
    khz / 1e6
}

pub fn khz_to_mhz(khz: f64) -> f64 {
    khz / 1e3
}

pub fn khz_to_hz(khz: f64) -> f64 {
    khz * 1e3
}

pub fn hz_to_thz(hz: f64) -> f64 {
    hz / 1e12
}

pub fn hz_to_ghz(hz: f64) -> f64 {
    hz / 1e9
}

pub fn hz_to_mhz(hz: f64) -> f64 {
    hz / 1e6
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
        // Values chosen such that they all convert to 1 THz or 1,000,000,000,000 Hz
        let thz = 1.0;
        let ghz = 1000.0;
        let mhz = 1_000_000.0;
        let khz = 1_000_000_000.0;
        let hz = 1_000_000_000_000.0;
        assert_eq!(super::thz_to_ghz(thz), ghz);
        assert_eq!(super::thz_to_mhz(thz), mhz);
        assert_eq!(super::thz_to_khz(thz), khz);
        assert_eq!(super::thz_to_hz(thz), hz);

        assert_eq!(super::ghz_to_thz(ghz), thz);
        assert_eq!(super::ghz_to_mhz(ghz), mhz);
        assert_eq!(super::ghz_to_khz(ghz), khz);
        assert_eq!(super::ghz_to_hz(ghz), hz);

        assert_eq!(super::mhz_to_thz(mhz), thz);
        assert_eq!(super::mhz_to_ghz(mhz), ghz);
        assert_eq!(super::mhz_to_khz(mhz), khz);
        assert_eq!(super::mhz_to_hz(mhz), hz);

        assert_eq!(super::khz_to_thz(khz), thz);
        assert_eq!(super::khz_to_ghz(khz), ghz);
        assert_eq!(super::khz_to_mhz(khz), mhz);
        assert_eq!(super::khz_to_hz(khz), hz);

        assert_eq!(super::hz_to_thz(hz), thz);
        assert_eq!(super::hz_to_ghz(hz), ghz);
        assert_eq!(super::hz_to_mhz(hz), mhz);
        assert_eq!(super::hz_to_khz(hz), khz);
    }

    #[test]
    fn wavelength_to_frequency_roundtrip_one_gigahertz() {
        let base: f64 = 10.0;
        let frequency: f64 = 1.0 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);
        let result: f64 = super::wavelength_to_frequency(wavelength);

        assert_eq!(frequency, result);
    }

    #[test]
    fn wavelength_to_frequency_ka_band_30_ghz() {
        let base: f64 = 10.0;
        let frequency: f64 = 30.0 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);
        assert_eq!(0.009993081933333333, wavelength);

        let result: f64 = super::wavelength_to_frequency(wavelength);
        assert_eq!(frequency, result);
    }

}
