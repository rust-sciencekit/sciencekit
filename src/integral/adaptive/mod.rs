use std::cmp::Ordering;

pub mod romberg;
pub use romberg::*;

pub mod trapezoidal;
pub use trapezoidal::*;

pub mod simpson;
pub use simpson::*;

pub mod gauss_kronrod;
pub use gauss_kronrod::*;

struct Interval {
    a: f64,
    b: f64,
    value: f64,
    error: f64,
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.error.to_bits() == other.error.to_bits()
    }
}

impl Eq for Interval {}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.error.total_cmp(&other.error)
    }
}

#[cfg(test)]
mod romberg_method {

    use crate::integral;
    use crate::measure::*;

    #[test]
    fn simple_integral_1() {
        let f = |x: f64| -> f64 { x.powi(4) - 2.0 * x + 1.0 };
        let result: Measure = integral::adaptive::romberg(f, 0.0, 2.0, 10, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 4.4).abs() <= 0.0001 && (result.value - 4.4).abs() <= result.error,
            "Adaptive Romberg integral gives wrong result!"
        );
    }

    #[test]
    fn simple_integral_2() {
        let f = |x: f64| -> f64 { (100.0 * x).sqrt().sin().powi(2) };
        let result: Measure = integral::adaptive::romberg(f, 0.0, 1.0, 10, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 0.45583).abs() <= 0.0001
                && (result.value - 0.45583).abs() <= result.error,
            "Adaptive Romberg integral gives wrong result!"
        );
    }
}

#[cfg(test)]
mod trapezoidal_method {

    use crate::integral;
    use crate::measure::*;

    #[test]
    fn simple_integral_1() {
        let f = |x: f64| -> f64 { x.powi(4) - 2.0 * x + 1.0 };
        let result: Measure = integral::adaptive::trapezoidal(f, 0.0, 2.0, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 4.4).abs() <= 0.0001 && (result.value - 4.4).abs() <= result.error,
            "Adaptive trapezoidal integral gives wrong result!"
        );
    }

    #[test]
    fn simple_integral_2() {
        let f = |x: f64| -> f64 { (100.0 * x).sqrt().sin().powi(2) };
        let result: Measure = integral::adaptive::trapezoidal(f, 0.0, 1.0, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 0.45583).abs() <= 0.0001
                && (result.value - 0.45583).abs() <= result.error,
            "Adaptive trapezoidal integral gives wrong result!"
        );
    }
}

#[cfg(test)]
mod simpson_method {

    use crate::integral;
    use crate::measure::*;

    #[test]
    fn simple_integral_1() {
        let f = |x: f64| -> f64 { x.powi(4) - 2.0 * x + 1.0 };
        let result: Measure = integral::adaptive::simpson(f, 0.0, 2.0, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 4.4).abs() <= 0.0001 && (result.value - 4.4).abs() <= result.error,
            "Adaptive simpsons integral gives wrong result!"
        );
    }

    #[test]
    fn simple_integral_2() {
        let f = |x: f64| -> f64 { (100.0 * x).sqrt().sin().powi(2) };
        let result: Measure = integral::adaptive::simpson(f, 0.0, 1.0, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 0.45583).abs() <= 0.0001
                && (result.value - 0.45583).abs() <= result.error,
            "Adaptive simpson integral gives wrong result!"
        );
    }
}

#[cfg(test)]
mod gauss_kronrod_method {

    use crate::integral;
    use crate::measure::*;

    #[test]
    fn simple_integral_1() {
        let f = |x: f64| -> f64 { x.powi(4) - 2.0 * x + 1.0 };
        let result: Measure = integral::adaptive::gauss_kronrod(f, 0.0, 2.0, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 4.4).abs() <= 0.0001 && (result.value - 4.4).abs() <= result.error,
            "Adaptive Gauss-Kronrod integral gives wrong result!"
        );
    }

    #[test]
    fn simple_integral_2() {
        let f = |x: f64| -> f64 { (100.0 * x).sqrt().sin().powi(2) };
        let result: Measure = integral::adaptive::gauss_kronrod(f, 0.0, 1.0, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 0.45583).abs() <= 0.0001 && result.error <= 0.0001,
            "Adaptive Gauss-Kronrod integral gives wrong result!"
        );
    }
}
