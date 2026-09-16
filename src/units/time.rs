use crate::units::Time::*;

pub enum Time {
    Picosecond,
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Kilosecond,
    Minute,
    Hour,
    Day,
    SideralDay,
    Week,
    Year,
    TropicalYear,
    SideralYear,
}

pub fn convert_time(value: f64, old_unit: Time, exponent: f64, new_unit: Time) -> f64 {
    // We first convert the value in seconds
    let value_in_second = match old_unit {
        Picosecond => value * (1.0e-12_f64).powf(exponent),
        Nanosecond => value * (1.0e-9_f64).powf(exponent),
        Microsecond => value * (1.0e-6_f64).powf(exponent),
        Millisecond => value * (1.0e-3_f64).powf(exponent),
        Second => value,
        Kilosecond => value * (1.0e3_f64).powf(exponent),
        Minute => value * (60_f64).powf(exponent),
        Hour => value * (3600_f64).powf(exponent),
        Day => value * (86_400_f64).powf(exponent),
        SideralDay => value * (86_164.1_f64).powf(exponent),
        Week => value * (604_800_f64).powf(exponent),
        Year => value * (3.154e7_f64).powf(exponent),
        TropicalYear => value * (31_556_926.08_f64).powf(exponent),
        SideralYear => value * (31_556_925.129_6_f64).powf(exponent),
    };

    // Then we convert it into the unit we really want
    match new_unit {
        Picosecond => value_in_second / (1.0e-12_f64).powf(exponent),
        Nanosecond => value_in_second / (1.0e-9_f64).powf(exponent),
        Microsecond => value_in_second / (1.0e-6_f64).powf(exponent),
        Millisecond => value_in_second / (1.0e-3_f64).powf(exponent),
        Second => value_in_second,
        Kilosecond => value_in_second / (1.0e3_f64).powf(exponent),
        Minute => value_in_second / (60_f64).powf(exponent),
        Hour => value_in_second / (3600_f64).powf(exponent),
        Day => value_in_second / (86_400_f64).powf(exponent),
        SideralDay => value_in_second / (86_164.1_f64).powf(exponent),
        Week => value_in_second / (604_800_f64).powf(exponent),
        Year => value_in_second / (3.154e7_f64).powf(exponent),
        TropicalYear => value_in_second / (31_556_926.08_f64).powf(exponent),
        SideralYear => value_in_second / (31_556_925.129_6_f64).powf(exponent),
    }
}
