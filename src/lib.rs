#![warn(missing_docs)]
//! RF engineering unit conversions for power, frequency, noise, and compression point analysis.

/// Physical constants used by the conversion routines.
pub mod constants;
/// Frequency and wavelength conversions.
pub mod frequency;
/// Noise figure, noise factor, noise temperature, and thermal noise conversions.
pub mod noise;
/// P1dB compression point conversion helpers.
pub mod p1db;
/// Power conversions including watts, dBm, dBW, and linear ratios.
pub mod power;
