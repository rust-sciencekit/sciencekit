use crate::linear_algebra::vector::*;

pub fn linear_space(start: f64, end: f64, n: usize) -> Vector {
    match n {
        0 => Vector::from_components(vec![]),
        1 => Vector::from_components(vec![start]),
        _ => {
            let step = (end - start) / (n as f64 - 1.0);
            let values: Vec<f64> = (0..n).map(|i| start + i as f64 * step).collect();
            Vector::from_components(values)
        }
    }
}
