pub mod trapezoidal;
pub use trapezoidal::*;

pub mod simpson;
pub use simpson::*;

pub mod gaussian;
pub use gaussian::*;

#[cfg(test)]
mod trapezoidal_rule {

    use crate::integral;
    use crate::measure::*;

    #[test]
    fn simple_integral() {
        let f = |x: f64| -> f64 { x.powi(4) - 2.0 * x + 1.0 };
        let result: Measure = integral::non_adaptive::trapezoidal(f, 0.0, 2.0, 1000);
        assert!(
            (result.value - 4.4).abs() <= 0.0001 && (result.value - 4.4).abs() <= result.error,
            "Trapezoidal integral gives wrong result!"
        );
    }
}

#[cfg(test)]
mod simpson_rule {

    use crate::integral;
    use crate::measure::*;

    #[test]
    fn simple_integral() {
        let f = |x: f64| -> f64 { x.powi(4) - 2.0 * x + 1.0 };
        let result: Measure = integral::non_adaptive::simpson(f, 0.0, 2.0, 10);
        assert!(
            (result.value - 4.4).abs() <= 0.001 && (result.value - 4.4).abs() <= result.error,
            "Trapezoidal integral gives wrong result!"
        );
    }
}

#[cfg(test)]
mod gaussian_integral {

    use crate::integral;

    #[test]
    fn simple_integral() {
        let f = |x: f64| -> f64 { x.powi(4) - 2.0 * x + 1.0 };
        let result: f64 = integral::non_adaptive::gaussian(f, 0.0, 2.0, 10);
        println!("{result}");
        assert!(
            (result - 4.4).abs() <= 0.0000001,
            "Gaussian integral gives wrong result!"
        );
    }
}
