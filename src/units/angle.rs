use crate::units::Angle::*;

pub enum Angle {
    Turn,
    Arcsecond,
    Arcminute,
    Degree,
    Gradian,
    Radian,
    Milliradian,
}

pub fn convert_angle(value: f64, old_unit: Angle, exponent: f64, new_unit: Angle) -> f64 {
    // We first convert the value in turns
    let value_in_turns = match old_unit {
        Turn => value,
        Arcsecond => value / (1_296_000_f64).powf(exponent),
        Arcminute => value / (std::f64::consts::TAU).powf(exponent),
        Degree => value / (360_f64).powf(exponent),
        Gradian => value / (400_f64).powf(exponent),
        Radian => value / (std::f64::consts::TAU).powf(exponent),
        Milliradian => value / (std::f64::consts::TAU * 1000.0).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match new_unit {
        Turn => value_in_turns,
        Arcsecond => value_in_turns * (1_296_000_f64).powf(exponent),
        Arcminute => value_in_turns * (std::f64::consts::TAU).powf(exponent),
        Degree => value_in_turns * (360_f64).powf(exponent),
        Gradian => value_in_turns * (400_f64).powf(exponent),
        Radian => value_in_turns * (std::f64::consts::TAU).powf(exponent),
        Milliradian => value_in_turns * (std::f64::consts::TAU * 1000.0).powf(exponent),
    }
}
