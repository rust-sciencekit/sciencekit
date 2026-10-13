pub mod trapezoidal;
pub use trapezoidal::*;

#[cfg(test)]
mod trapezoidal_rule {

    use crate::integral;

    #[test]
    fn simple_integral() {
        let x: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let y: Vec<f64> = vec![0.0, 2.0, 0.0, 2.0, 0.0];
        let result: f64 = integral::tabulated::trapezoidal(x, y);
        assert!(
            result == 4.0,
            "Trapezoidal integral gives wrong result on tabulated data!"
        );
    }

    #[test]
    fn uneven_x_axis() {
        let x: Vec<f64> = vec![0.0, 0.5, 1.0, 2.0, 4.0];
        let y: Vec<f64> = vec![0.0, 1.0, 2.0, 0.0, 2.0];
        let result: f64 = integral::tabulated::trapezoidal(x, y);
        assert!(
            result == 4.0,
            "Trapezoidal integral gives wrong result on tabulated data with uneven x axis!"
        );
    }

    #[test]
    fn unsorted_x_axis() {
        let x: Vec<f64> = vec![4.0, 0.5, 0.0, 1.0, 2.0];
        let y: Vec<f64> = vec![2.0, 1.0, 0.0, 2.0, 0.0];
        let result: f64 = integral::tabulated::trapezoidal(x, y);
        assert!(
            result == 4.0,
            "Trapezoidal integral gives wrong result on tabulated data with unsorted x axis!"
        );
    }
}
