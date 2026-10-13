use crate::measure::*;

struct Interval {
    f: fn(f64) -> f64,
    lower_bound: f64,
    n_slices: usize,
    h: f64,
    /// The integral estimate of the previous Romberg step
    previous_values: Vec<f64>,
    /// The integral estimates of the last Romberg step
    values: Vec<f64>,
    error: f64,
    romberg_level: usize,
}

impl Interval {
    fn new(a: f64, b: f64, f: fn(f64) -> f64) -> Interval
//where F: Fn(f64) -> f64
    {
        let lower_bound: f64 = a.min(b);
        let upper_bound: f64 = a.max(b);

        // We start with a one slice trapezoidal integration
        let mut h: f64 = upper_bound - lower_bound;
        let previous_values: Vec<f64> = vec![(f(upper_bound) + f(lower_bound)) * h / 2.0];

        // Then we add a 2 slices trapezoidal integration + Romberg refinement
        h /= 2.0;

        let mut values: Vec<f64> = vec![previous_values[0] / 2.0 + h * f(lower_bound + h)];
        values.push(values[0] + (1.0 / 3.0) * (values[0] - previous_values[0]));

        // Error estimation
        let error: f64 = ((1.0 / (4.0_f64.powi(2) - 1.0)) * (values[0] - previous_values[0])).abs();

        Interval {
            f,
            lower_bound,
            n_slices: 2,
            h,
            previous_values,
            values,
            error,
            romberg_level: 2,
        }
    }

    fn add_romberg_step(&mut self) {
        let mut new_values: Vec<f64> = Vec::new();

        self.n_slices *= 2;
        self.h /= 2.0;

        // First trapezoidal estimation
        let mut first_estimate: f64 = self.values[0] / 2.0;

        let mut temp: f64 = 0.0;
        for k in (1..self.n_slices).step_by(2) {
            temp += (self.f)(self.lower_bound + (k as f64) * self.h);
        }

        first_estimate += temp * self.h;

        new_values.push(first_estimate);

        // Adding Romberg refinements
        for k in 1..=self.romberg_level {
            new_values.push(
                new_values[k - 1]
                    + (1.0 / (4.0_f64.powi((k) as i32) - 1.0))
                        * (new_values[k - 1] - self.values[k - 1]),
            );
        }

        // Error estimation
        self.error = ((1.0 / (4.0_f64.powi((self.romberg_level + 1) as i32) - 1.0))
            * (new_values[self.romberg_level] - self.values[self.romberg_level - 1]))
            .abs();

        self.romberg_level += 1;
        self.previous_values = self.values.clone();
        self.values = new_values;
    }
}

pub fn romberg(
    f: fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
    abs_error: f64,
    rel_error: f64,
) -> Measure
//where F: Fn(f64) -> f64
{
    if a == b {
        return Measure {
            value: 0.0,
            error: 0.0,
        };
    };

    let lower_bound: f64 = a.min(b);
    let upper_bound: f64 = a.max(b);

    let h: f64 = (upper_bound - lower_bound) / n as f64;

    // Initializing the intervals
    let mut intervals: Vec<Interval> = Vec::new();

    for k in 0..n {
        let a: f64 = lower_bound + k as f64 * h;
        let b: f64 = lower_bound + (k + 1) as f64 * h;

        intervals.push(Interval::new(a, b, f));
    }

    let mut final_result: f64 = intervals.iter().map(|i| i.values.last().unwrap()).sum();

    let mut final_error: f64 = intervals.iter().map(|i| i.error).sum();

    while final_error >= abs_error && final_error >= rel_error * final_result.abs() {
        let largest_error_index = intervals
            .iter()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.error.total_cmp(&y.error))
            .map(|(i, _)| i)
            .unwrap();

        intervals[largest_error_index].add_romberg_step();

        // Updating global result and error
        final_result = intervals.iter().map(|i| i.values.last().unwrap()).sum();

        final_error = intervals.iter().map(|i| i.error).sum();
    }

    Measure {
        value: final_result,
        error: final_error,
    }
}
