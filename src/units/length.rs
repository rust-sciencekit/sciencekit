use crate::units::Length::*;

pub enum Length {
    Kilometer,
    Meter,
    Centimeter,
    Millimeter,
    Micrometer,
    Nanometer,
    Angstrom,
    Miles,
    Yard,
    Feet,
    Inch,
    NauticalMiles,
}

pub fn convert_length(value: f64, old_unit: Length, exponent: f64, new_unit: Length) -> f64 {
    // We first convert the value in meters
    let value_in_meters = match old_unit {
        Kilometer => value * (1000_f64).powf(exponent),
        Meter => value,
        Centimeter => value * (0.01_f64).powf(exponent),
        Millimeter => value * (0.001_f64).powf(exponent),
        Micrometer => value * (1.0e-6_f64).powf(exponent),
        Nanometer => value * (1.0e-9_f64).powf(exponent),
        Angstrom => value * (1.0e-10_f64).powf(exponent),
        Miles => value * (1609.34_f64).powf(exponent),
        Yard => value * (0.9144_f64).powf(exponent),
        Feet => value * (0.3048_f64).powf(exponent),
        Inch => value * (0.0254_f64).powf(exponent),
        NauticalMiles => value * (1852_f64).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match new_unit {
        Kilometer => value_in_meters / (1000_f64).powf(exponent),
        Meter => value_in_meters,
        Centimeter => value_in_meters / (0.01_f64).powf(exponent),
        Millimeter => value_in_meters / (0.001_f64).powf(exponent),
        Micrometer => value_in_meters / (1.0e-6_f64).powf(exponent),
        Nanometer => value_in_meters / (1.0e-9_f64).powf(exponent),
        Angstrom => value_in_meters / (1.0e-10_f64).powf(exponent),
        Miles => value_in_meters / (1609.34_f64).powf(exponent),
        Yard => value_in_meters / (0.9144_f64).powf(exponent),
        Feet => value_in_meters / (0.3048_f64).powf(exponent),
        Inch => value_in_meters / (0.0254_f64).powf(exponent),
        NauticalMiles => value_in_meters / (1852_f64).powf(exponent),
    }
}
