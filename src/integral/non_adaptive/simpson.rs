use crate::measure::*;

pub fn simpson<F>(f: F, a: f64, b: f64, n: usize) -> Measure
where
    F: Fn(f64) -> f64,
{
    if a == b {
        return Measure {
            value: 0.0,
            error: 0.0,
        };
    };

    // Simpson integration requires an even number of slices, both for n steps and n/2 steps
    let mut n: usize = if n.is_multiple_of(4) {
        n
    } else {
        n + (4 - n % 4)
    };

    let lower_bound: f64 = if a < b { a } else { b };
    let upper_bound: f64 = if a < b { b } else { a };

    // First estimate of the integral with half the number of steps
    n /= 2;
    let mut h: f64 = (upper_bound - lower_bound) / (n as f64);

    let mut simpson_estimate: f64 = f(lower_bound) + f(upper_bound);

    let mut temp: f64 = 0.0;
    for k in (2..n - 1).step_by(2) {
        temp += f(lower_bound + (k as f64) * h);
    }
    simpson_estimate += 2.0 * temp;
    simpson_estimate /= 3.0;

    let mut trapezoid_estimate: f64 = 0.0;
    for k in (1..n).step_by(2) {
        trapezoid_estimate += f(lower_bound + (k as f64) * h);
    }
    trapezoid_estimate *= 2.0 / 3.0;

    let first_estimate: f64 = h * (simpson_estimate + 2.0 * trapezoid_estimate);

    // Second estimate of the integral
    n *= 2;
    h /= 2.0;

    simpson_estimate += trapezoid_estimate;

    trapezoid_estimate = 0.0;
    for k in (1..n).step_by(2) {
        trapezoid_estimate += f(lower_bound + (k as f64) * h);
    }
    trapezoid_estimate *= 2.0 / 3.0;

    let second_estimate: f64 = h * (simpson_estimate + 2.0 * trapezoid_estimate);

    // Estimation of the error
    let error: f64 = ((second_estimate - first_estimate) / 15.0).abs();

    Measure {
        value: second_estimate,
        error,
    }
}
