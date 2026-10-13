use std::ops::Mul;

use crate::linear_algebra::*;

/// Matrix * Vector
impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        if self.cols != rhs.components.len() {
            panic!("Incompatible matrix and vector dimensions");
        }

        let mut components = vec![0.0; self.rows];
        for (i, component) in components.iter_mut().enumerate().take(self.rows) {
            for j in 0..self.cols {
                *component += self.components[i * self.cols + j] * rhs.components[j];
            }
        }
        Vector { components }
    }
}

impl Mul<&Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        if self.cols != rhs.components.len() {
            panic!("Incompatible matrix and vector dimensions");
        }

        let mut components = vec![0.0; self.rows];
        for (i, component) in components.iter_mut().enumerate().take(self.rows) {
            for j in 0..self.cols {
                *component += self.components[i * self.cols + j] * rhs.components[j];
            }
        }
        Vector { components }
    }
}

impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        if self.cols != rhs.components.len() {
            panic!("Incompatible matrix and vector dimensions");
        }

        let mut components = vec![0.0; self.rows];
        for (i, component) in components.iter_mut().enumerate().take(self.rows) {
            for j in 0..self.cols {
                *component += self.components[i * self.cols + j] * rhs.components[j];
            }
        }
        Vector { components }
    }
}

impl Mul<&Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        if self.cols != rhs.components.len() {
            panic!("Incompatible matrix and vector dimensions");
        }

        let mut components = vec![0.0; self.rows];
        for (i, component) in components.iter_mut().enumerate().take(self.rows) {
            for j in 0..self.cols {
                *component += self.components[i * self.cols + j] * rhs.components[j];
            }
        }
        Vector { components }
    }
}
