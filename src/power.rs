pub fn watts_to_dbm(watts: f64) -> f64 {
    10.0 * (watts.log10() + 3.0)
}

pub fn dbm_to_watts(dbm: f64) -> f64 {
    10.0_f64.powf((dbm - 30.0) / 10.0)
}

pub fn db_to_linear(value: f64) -> f64 {
    10.0_f64.powf(value / 10.0)
}

pub fn linear_to_db(value: f64) -> f64 {
    10.0 * f64::log10(value)
}

#[cfg(test)]
mod tests {

    #[test]
    fn watts_to_dbm() {
        let watts: f64 = 1.0;

        let dbm: f64 = super::watts_to_dbm(watts);

        assert_eq!(30.0, dbm);
    }

    #[test]
    fn another_watts_to_dbm() {
        let watts: f64 = 20.0;

        let dbm: f64 = super::watts_to_dbm(watts);

        // not worrying about floating point precision here
        assert_eq!(43.01029995663981, dbm);
    }

    #[test]
    fn dbm_to_watts() {
        // not worrying about floating point precision here
        let dbm: f64 = 43.010_299_956_639_805;

        let watts: f64 = super::dbm_to_watts(dbm);

        // not worrying about floating point precision here
        assert_eq!(19.99999999999997, watts);
    }

    #[test]
    fn another_dbm_to_watts() {
        let dbm: f64 = 30.0;

        let watts: f64 = super::dbm_to_watts(dbm);

        assert_eq!(1.0, watts);
    }

    #[test]
    fn db_to_linear() {
        let db: f64 = 30.0;

        let linear: f64 = super::db_to_linear(db);

        assert_eq!(1000.0, linear);
    }

    #[test]
    fn another_db_to_linear() {
        let db: f64 = -10.0;

        let linear: f64 = super::db_to_linear(db);

        assert_eq!(0.1, linear);
    }
    #[test]
    fn another_db_to_linear_2() {
        let db: f64 = -13.0;

        let linear: f64 = super::db_to_linear(db);

        // -3.0 dB isn't exactly half
        // therefore -13 dB isn't exactly 1/20 
        assert_eq!(0.05011872336272722, linear);
    }

    #[test]
    fn linear_to_db() {
        let linear: f64 = 1000.0;

        let db: f64 = super::linear_to_db(linear);

        assert_eq!(30.0, db);
    }

    #[test]
    fn another_linear_to_db() {
        let linear: f64 = 0.1;

        let db: f64 = super::linear_to_db(linear);

        assert_eq!(-10.0, db);
    }
    #[test]
    fn another_linear_to_db_2() {
        let linear: f64 = 0.05011872336272722;

        let db: f64 = super::linear_to_db(linear);

        // -3.0 dB isn't exactly half
        // therefore -13 dB isn't exactly 1/20 
        assert_eq!(-13.0, db);
    }
}
