use crate::units::Temperature::*;

pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub fn convert_temperature(
    value: f64,
    old_unit: Temperature,
    exponent: f64,
    new_unit: Temperature,
) -> f64 {
    // Removing the exponent
    let value_without_exponent: f64 = value.powf(1.0 / exponent);

    // We then convert the value in kelvin
    let value_in_kelvin = match old_unit {
        Kelvin => value_without_exponent,
        Celsius => value_without_exponent + 273.15,
        Fahrenheit => (value_without_exponent - 32.0) * 5.0 / 9.0 + 273.15,
    };

    // we convert it into the unit we really want
    let result = match new_unit {
        Kelvin => value_in_kelvin,
        Celsius => value_in_kelvin - 273.15,
        Fahrenheit => (value_in_kelvin - 273.15) * 9.0 / 5.0 + 32.0,
    };

    result.powf(exponent)
}
