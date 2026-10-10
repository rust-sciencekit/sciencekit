#[macro_use]
pub mod real_polynomial;
pub use real_polynomial::*;

#[cfg(test)]
mod polynomial_creation {

    use crate::polynomial::*;

    #[test]
    fn using_macro() {
        let p: Polynomial = polynomial![3, 2, 1];

        assert!(
            p.evaluate(-1.0) == 2.0 && p.evaluate(0.0) == 3.0 && p.evaluate(1.0) == 6.0,
            "the command polynomial![3, 2, 1] should return 3 + 2x + x² and not {}",
            p
        );
    }

    #[test]
    fn empty_macro() {
        let p: Polynomial = polynomial![];

        assert!(
            p.evaluate(-1.0) == 0.0 && p.evaluate(0.0) == 0.0 && p.evaluate(1.0) == 0.0,
            "the command polynomial![] should return 0 and not {}",
            p
        );
    }

    #[test]
    fn new() {
        let p = Polynomial::new();

        assert!(
            p.evaluate(-1.0) == 0.0 && p.evaluate(0.0) == 0.0 && p.evaluate(1.0) == 0.0,
            "the function Polynomial::new should return 0 and not {}",
            p
        );
    }
}

#[cfg(test)]
mod polynomial_evaluation {

    use crate::{complex::Complex, polynomial::*};

    #[test]
    fn real_evaluation() {
        let p: Polynomial = polynomial![1, 2, 3];
        assert!(
            p.evaluate(3.0) == 34.0,
            "3x² + 2x + 1 at x = 3 should give 34 and not {}",
            p.evaluate(3.0)
        );
    }

    #[test]
    fn complex_evaluation() {
        let p: Polynomial = polynomial![1, 2, 3];
        let z: Complex = Complex { re: 1.0, im: 2.0 };
        assert!(
            p.evaluate_complex(z) == Complex { re: -6.0, im: 16.0 },
            "3x² + 2x + 1 at x = (1 + 2i) should give (-6 + 16i) and not {}",
            p.evaluate_complex(z)
        );
    }
}
