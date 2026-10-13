use std::f64::consts::PI;

use crate::{linear_algebra::Vector, linear_space};

fn gaussian_xs_ws(lower_bound: f64, upper_bound: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    // Initial approximation to roots of the Legendre polynomial
    let a: Vector = linear_space(3.0, 4.0 * n as f64 - 1.0, n) / (4.0 * n as f64 + 2.0);

    let mut x: Vector = a.clone();
    x.apply_function(|x| (PI * x + 1.0 / (8.0 * n.pow(2) as f64 * x.tan())).cos());

    // Find roots using Newton's method
    let mut delta: f64 = 1.0;

    let mut p0: Vector = Vector::from_values(1.0, n);
    while delta > f64::EPSILON {
        p0.set_all(1.0);
        let mut p1: Vector = x.clone();

        for k in 1..n {
            for i in 0..n {
                let temp: f64 = p0.get(i);
                p0.set(i, p1.get(i));
                p1.set(
                    i,
                    ((2.0 * k as f64 + 1.0) * x.get(i) * p1.get(i) - k as f64 * temp)
                        / (k as f64 + 1.0),
                );
            }
        }

        let dp: Vector = (n + 1) as f64 * (&p0 - &x * &p1) / (1.0 - &x * &x);
        let dx = &p1 / &dp;
        x = x - &dx;

        delta = dx.components[dx.index_of_absolute_max()].abs();
    }

    // Calculating the weights
    let mut weights: Vector = 2.0 * (1.0 - &x * &x) / (&p0 * &p0 * n.pow(2) as f64);

    weights = 0.5 * (upper_bound - lower_bound) * weights;
    x = 0.5 * (upper_bound - lower_bound) * x + 0.5 * (upper_bound + lower_bound);

    (x.components, weights.components)
}

pub fn gaussian(f: fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let (xs, ws) = gaussian_xs_ws(a, b, n);

    let mut result: f64 = 0.0;

    for i in 0..xs.len() {
        result += f(xs[i]) * ws[i];
    }

    result
}
