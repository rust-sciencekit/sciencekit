use crate::measure::*;

pub fn trapezoidal<F>(f: F, a: f64, b: f64, n: usize) -> Measure
where
    F: Fn(f64) -> f64,
{
    if a == b {
        return Measure {
            value: 0.0,
            error: 0.0,
        };
    };

    let mut n: usize = if n.is_multiple_of(2) { n } else { n + 1 };

    let lower_bound: f64 = if a < b { a } else { b };
    let upper_bound: f64 = if a < b { b } else { a };

    // First estimation of the integral with half the steps
    n /= 2;
    let mut h: f64 = (upper_bound - lower_bound) / (n as f64);

    let mut first_estimate: f64 = (f(lower_bound) + f(upper_bound)) / 2.0;

    for k in 1..n {
        first_estimate += f(lower_bound + (k as f64) * h);
    }

    first_estimate *= h;

    // Second estimate of the integral, this time with the right amount of steps
    n *= 2;
    h /= 2.0;

    let mut second_estimate: f64 = first_estimate / 2.0;

    let mut temp: f64 = 0.0;
    for k in (1..n).step_by(2) {
        temp += f(lower_bound + (k as f64) * h);
    }
    second_estimate += temp * h;

    // Estimation of the error with a factor 1.1 to avoid underestimation of the error
    let error: f64 = 1.1 * ((second_estimate - first_estimate) / 3.0).abs();

    Measure {
        value: second_estimate,
        error,
    }
}
