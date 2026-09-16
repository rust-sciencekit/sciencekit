pub mod time;
pub use time::*;

pub mod length;
pub use length::*;

pub mod energy;
pub use energy::*;

pub mod mass;
pub use mass::*;

pub mod angle;
pub use angle::*;

pub mod temperature;
pub use temperature::*;

#[cfg(test)]
mod time_conversion {

    use crate::units;

    #[test]
    fn from_seconds_to_hours() {
        let time_in_seconds: f64 = 1800.0;
        let time_in_hours: f64 =
            units::convert_time(time_in_seconds, units::Time::Second, 1.0, units::Time::Hour);

        assert!(
            time_in_hours == 0.5,
            "1800 seconds makes {time_in_hours} hours instead of 0.5!"
        );
    }

    #[test]
    fn speed_conversion() {
        let c_seconds: f64 = crate::constants::f64::SPEED_OF_LIGHT;
        let c_minutes: f64 =
            units::convert_time(c_seconds, units::Time::Second, -1.0, units::Time::Minute);

        assert!(
            c_minutes == 17_987_547_480_f64,
            "The speed of light in m/min should be 17_987_547_480 and not {c_minutes}!"
        );
    }
}

#[cfg(test)]
mod length_conversion {

    use crate::units;

    #[test]
    fn surface_conversion() {
        let square_meters: f64 = 150.0;
        let square_foot: f64 = units::convert_length(
            square_meters,
            units::Length::Meter,
            2.0,
            units::Length::Feet,
        );

        assert!(
            (square_foot - 1614.59).abs() < 0.01,
            "150 m² should be about 1614.59 ft² and not {square_foot}!"
        );
    }
}

#[cfg(test)]
mod energy_conversion {

    use crate::units;

    #[test]
    fn from_joules_to_ev() {
        let joules: f64 = 100.0;
        let electron_volt: f64 = units::convert_energy(
            joules,
            units::Energy::Joule,
            1.0,
            units::Energy::Electronvolt,
        );

        assert!(
            (electron_volt - 6.2415e20).abs() < 0.01e20,
            "100 Joules should be about 6.2415e20 eV and not {electron_volt}!"
        );
    }
}

#[cfg(test)]
mod mass_conversion {

    use crate::units;

    #[test]
    fn from_grams_to_pounds() {
        let gram: f64 = 100.0;
        let pound: f64 = units::convert_mass(gram, units::Mass::Gram, 1.0, units::Mass::Pound);

        assert!(
            (pound - 0.22).abs() < 0.01,
            "100 grams should be about 0.22 pounds and not {pound}!"
        );
    }
}

#[cfg(test)]
mod angle_conversion {

    use crate::units;

    #[test]
    fn pi_radians_in_degrees() {
        let angle_radian: f64 = std::f64::consts::PI;
        let angle_degree: f64 = units::convert_angle(
            angle_radian,
            units::Angle::Radian,
            1.0,
            units::Angle::Degree,
        );

        assert!(
            angle_degree == 180.0,
            "π radians should be equal to 180 degrees and not {angle_degree}!"
        );
    }
}

#[cfg(test)]
mod temperature_conversion {

    use crate::units;

    #[test]
    fn from_celsius_to_fahrenheit() {
        let celsius: f64 = 32.0;
        let fahrenheit: f64 = units::convert_temperature(
            celsius,
            units::Temperature::Celsius,
            1.0,
            units::Temperature::Fahrenheit,
        );

        assert!(
            fahrenheit == 89.6,
            "32 celsius should be equal to 98.6 Fahrenheit and not {fahrenheit}!"
        );
    }
}
