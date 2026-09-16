use crate::units::Mass::*;

pub enum Mass {
    Tonne,
    Kilogram,
    Gram,
    Milligram,
    Microgram,
    ImperialTon,
    USTon,
    Stone,
    Pound,
    Ounce,
}

pub fn convert_mass(value: f64, old_unit: Mass, exponent: f64, new_unit: Mass) -> f64 {
    // We first convert the value in grams
    let value_in_gram = match old_unit {
        Tonne => value * (1.0e6_f64).powf(exponent),
        Kilogram => value * (1000_f64).powf(exponent),
        Gram => value,
        Milligram => value * (0.001_f64).powf(exponent),
        Microgram => value * (1.0e-6_f64).powf(exponent),
        ImperialTon => value * (1.016e6_f64).powf(exponent),
        USTon => value * (907_185_f64).powf(exponent),
        Stone => value * (6_350.295_021_585_f64).powf(exponent),
        Pound => value * (453.592_501_541_785_7_f64).powf(exponent),
        Ounce => value * (28.349_531_346_361_605_f64).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match new_unit {
        Tonne => value_in_gram / (1.0e6_f64).powf(exponent),
        Kilogram => value_in_gram / (1000_f64).powf(exponent),
        Gram => value_in_gram,
        Milligram => value_in_gram / (0.001_f64).powf(exponent),
        Microgram => value_in_gram / (1.0e-6_f64).powf(exponent),
        ImperialTon => value_in_gram / (1.016e6_f64).powf(exponent),
        USTon => value_in_gram / (907_185_f64).powf(exponent),
        Stone => value_in_gram / (6_350.295_021_585_f64).powf(exponent),
        Pound => value_in_gram / (453.592_501_541_785_7_f64).powf(exponent),
        Ounce => value_in_gram / (28.349_531_346_361_605_f64).powf(exponent),
    }
}
