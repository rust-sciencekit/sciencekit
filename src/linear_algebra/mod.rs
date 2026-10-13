pub mod vector;
pub use vector::*;

pub mod matrix;
pub use matrix::*;

pub mod matrix_vector_operations;
//pub use matrix_vector_operations::*;

pub mod lu_decomposition;
pub use lu_decomposition::*;

pub mod qr_decomposition;

pub mod solve;
pub use solve::*;

#[cfg(test)]
mod matrix_creation {
    use crate::linear_algebra::{Matrix};

    #[test]
    fn new_default() {
        let a: Matrix = Matrix::new(2, 2);
        assert!(a.components == vec![0.0, 0.0, 0.0, 0.0] && a.cols == 2 && a.rows == 2, 
                "Creating a default 2x2 matrix should return [0, 0, 0, 0] and not {}", a);
    }

    #[test]
    fn new_from_components() {
        let a: Matrix = Matrix::from_components(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        assert!(a.components == vec![1.0, 2.0, 3.0, 4.0] && a.cols == 2 && a.rows == 2, 
                "Creating a 2x2 matrix from componens should return [1, 2, 3, 4] and not {}", a);
    }

    #[test]
    fn new_from_value() {
        let a: Matrix = Matrix::from_value(2.0, 2, 2);
        assert!(a.components == vec![2.0, 2.0, 2.0, 2.0] && a.cols == 2 && a.rows == 2, 
                "Creating a 2x2 matrix from value should return [2, 2, 2, 2] and not {}", a);
    }

    #[test]
    fn new_random() {
        let a: Matrix = Matrix::random(2, 2);
        let b: Matrix = Matrix::random(2, 2);
        let c: Matrix = Matrix::random(2, 2);
        assert!(a != b && b != c, "Random 2x2 matrices should not be the same!"); 
    }

    #[test]
    fn new_identity() {
        let a: Matrix = Matrix::identity(2);
        assert!(a.components == vec![1.0, 0.0, 0.0, 1.0], "Identity matrix should have 1 on the main diagonal and 0 elsewhere, not {}", a);
    }
}


#[cfg(test)]
mod getter {
    use crate::linear_algebra::{Matrix, Vector};

    #[test]
    fn single_element_get() {
        let a: Matrix = Matrix::from_components(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let test = a.get(0, 1);
        assert!(test == 2.0, "The matrix [1, 2, 3, 4] has 2 in row 0 and column 1 and not {}", test);
    }

    #[test]
    fn get_column() {
        let a: Matrix = Matrix::identity(3);
        let column: Vec<f64> = a.get_column(1);
        assert!(column == vec![0.0, 1.0, 0.0], "The second column of the 3x3 identity matrix should be [0, 1, 0] and not {:?}", column);
    }

    #[test]
    fn get_column_vector() {
        let a: Matrix = Matrix::identity(3);
        let test: Vector = a.get_column_vector(1);
        assert!(test == Vector::from_components(vec![0.0, 1.0, 0.0]), "The second column vector of the 3x3 identity matrix should be [0, 1, 0] and not {}", test);
    }

    #[test]
    fn get_row() {
        let a: Matrix = Matrix::from_components(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let test: Vec<f64> = a.get_row(1);
        assert!(test == vec![3.0, 4.0], "The second row of the matrix [1, 2, 3, 4] should be [3, 4] and not {:?}", test);
    }
}

#[cfg(test)]
mod trace_rank {
    use crate::linear_algebra::Matrix;

    #[test]
    fn trace() {
        let a: Matrix = Matrix::identity(5);
        let test: f64 = a.trace();
        assert!(test == 5.0, "The trace of a 5x5 identity matrix should be 5, not {}", test);
    }

    #[test]
    fn rank() {
        let a: Matrix = Matrix::from_components(vec![1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 1.0, 1.0, 1.0], 3, 3);
        let test: usize = a.rank();
        assert!(test == 2, "Wrong rank computation!");
    }
}
#[cfg(test)]
mod eigenvalues_eigenvectors {
    use crate::linear_algebra::{Matrix, Vector};

    #[test]
    fn eigenvalues() {
        let a: Matrix = Matrix::from_components(vec![1.0, 2.0, 2.0, 1.0], 2, 2);
        let eigenvalues: Vec<f64> = a.eigenvalues(1.0e-10);
        assert!((eigenvalues[0] - 3.0).abs() <= 1.0e-10, "Wrong eigenvalues computation. {:?} instead of [-1, 3]", eigenvalues);
    }

    #[test]
    fn eigenvectors() {
        let a: Matrix = Matrix::from_components(vec![1.0, 2.0, 2.0, 1.0], 2, 2);
        let absolute_error: f64 = 1.0e-10;
        let (eigenvectors, eigenvalues) = a.eigenvectors(absolute_error);
        
        assert!((eigenvalues[0] - 3.0).abs() <= absolute_error, "Wrong eigenvalues computation. {:?} instead of [3, -1]", eigenvalues);
        assert!((eigenvalues[1] - -1.0).abs() <= absolute_error, "Wrong eigenvalues computation. {:?} instead of [3, -1]", eigenvalues);

        let expected_eigenvector_1 = Vector::from_components(vec![0.70710678, 0.70710678]);
        let expected_eigenvector_2 = Vector::from_components(vec![0.70710678, -0.70710678]);

        assert!(eigenvectors[0].is_almost_equal(&expected_eigenvector_1, 1.0e-8), "Wrong eigenvector calculation: {} instead of [0.70710678, 0.70710678]", eigenvectors[0]);
        assert!(eigenvectors[1].is_almost_equal(&expected_eigenvector_2, 1.0e-8), "Wrong eigenvector calculation: {} instead of [0.70710678, -0.70710678]", eigenvectors[1]);
    }
}