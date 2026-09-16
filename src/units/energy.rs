use crate::units::Energy::*;

pub enum Energy {
    Joule,
    Kilojoule,
    GramCalorie,
    Kilocalorie,
    WattHour,
    KilowattHour,
    Electronvolt,
    BritishThermalUnit,
    USTherm,
    FootPound,
}

pub fn convert_energy(value: f64, old_unit: Energy, exponent: f64, new_unit: Energy) -> f64 {
    // We first convert the value in joule
    let value_in_joule = match old_unit {
        Joule => value,
        Kilojoule => value * (1000_f64).powf(exponent),
        GramCalorie => value * (4.184_f64).powf(exponent),
        Kilocalorie => value * (4184_f64).powf(exponent),
        WattHour => value * (3600_f64).powf(exponent),
        KilowattHour => value * (3.6e6_f64).powf(exponent),
        Electronvolt => value * (1.6022e-19_f64).powf(exponent),
        BritishThermalUnit => value * (1055.06_f64).powf(exponent),
        USTherm => value * (1.055e8_f64).powf(exponent),
        FootPound => value * (1.35582_f64).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match new_unit {
        Joule => value_in_joule,
        Kilojoule => value_in_joule / (1000_f64).powf(exponent),
        GramCalorie => value_in_joule / (4.184_f64).powf(exponent),
        Kilocalorie => value_in_joule / (4184_f64).powf(exponent),
        WattHour => value_in_joule / (3600_f64).powf(exponent),
        KilowattHour => value_in_joule / (3.6e6_f64).powf(exponent),
        Electronvolt => value_in_joule / (1.6022e-19_f64).powf(exponent),
        BritishThermalUnit => value_in_joule / (1055.06_f64).powf(exponent),
        USTherm => value_in_joule / (1.055e8_f64).powf(exponent),
        FootPound => value_in_joule / (1.35582_f64).powf(exponent),
    }
}
