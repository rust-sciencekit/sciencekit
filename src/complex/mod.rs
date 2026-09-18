#[allow(clippy::module_inception)]
pub mod complex;
pub use complex::*;

#[cfg(test)]
mod complex_creation {

    use super::*;

    #[test]
    fn create_empty_complex() {
        let z: Complex = Complex::new();

        assert!(
            z.re == 0.0 && z.im == 0.0,
            "Creating an empty complex number should return 0 + 0i and not {z}"
        );
    }

    #[test]
    fn from_cartesian() {
        let z = Complex::from_cartesian(2.0, 1.0);

        assert!(
            z.re == 2.0 && z.im == 1.0,
            "Creating a complex number from the cartesian coordinates (x, y) should return 2 + 1i and not {z}"
        );
    }

    #[test]
    fn from_polar() {
        let z = Complex::from_polar(2.0, std::f64::consts::FRAC_PI_2);

        assert!(
            z.re <= 1.0e-15 && z.im == 2.0,
            "Creating a complex number from the polar coordinates (2.0, π/2) should return 0 + 2i and not {z}"
        );
    }
}

#[cfg(test)]
mod complex_arithmetic {

    use super::*;

    #[test]
    fn addition() {
        let z1 = Complex { re: 1.0, im: 2.0 };
        let z2 = Complex { re: -2.0, im: 1.0 };

        assert!(
            z1 + z2 == Complex { re: -1.0, im: 3.0 },
            "(1 + 2i) + (-2 + 1i) should return -1 + 3i and not {}",
            z1 + z2
        );
    }

    #[test]
    fn substraction() {
        let z1 = Complex { re: 1.0, im: 2.0 };
        let z2 = Complex { re: -2.0, im: 1.0 };

        assert!(
            z1 - z2 == Complex { re: 3.0, im: 1.0 },
            "(1 + 2i) - (-2 + 1i) should return 3 + 1i and not {}",
            z1 - z2
        );
    }

    #[test]
    fn multiplication() {
        let z1 = Complex { re: 1.0, im: 2.0 };
        let z2 = Complex { re: -2.0, im: 1.0 };

        assert!(
            z1 * z2 == Complex { re: -4.0, im: -3.0 },
            "(1 + 2i) * (-2 + 1i) should return -4 - 3i and not {}",
            z1 * z2
        );
    }

    #[test]
    fn division() {
        let z1 = Complex { re: 1.0, im: 2.0 };
        let z2 = Complex { re: -2.0, im: 1.0 };

        assert!(
            z1 / z2 == Complex { re: 0.0, im: -1.0 },
            "(1 + 2i) / (-2 + 1i) should return -i and not {}",
            z1 * z2
        );
    }

    #[test]
    fn negation() {
        let z = Complex { re: 1.0, im: 2.0 };

        assert!(
            -z == Complex { re: -1.0, im: -2.0 },
            "-(1 + 2i) should return -1 - 2i and not {}",
            -z
        );
    }
}

#[cfg(test)]
mod trigonometric_functions {

    use super::*;

    #[test]
    fn sine() {
        let z1 = Complex { re: 0.0, im: 1.0 };

        assert!(
            z1.sin().re == 0.0 && (z1.sin().im - 1.1752).abs() <= 0.0001,
            "The sine of i should return about 0 + 1.1752i and not {}",
            z1.sin()
        );
    }

    #[test]
    fn cosine() {
        let z1 = Complex { re: 0.0, im: 1.0 };

        assert!(
            z1.cos().im == 0.0 && (z1.cos().re - 1.5431).abs() <= 0.0001,
            "The sine of i should return about 1.5431 and not {}",
            z1.cos()
        );
    }
}
