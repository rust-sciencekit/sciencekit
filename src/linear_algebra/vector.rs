use std::{
    fmt, ops::{Add, Div, DivAssign, Mul, Neg, Sub, SubAssign},
};

use crate::{linear_algebra::Matrix, permutation::Permutation};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub components: Vec<f64>,
}

impl Vector {
    pub fn new() -> Self {
        Vector { components: vec![] }
    }

    pub fn from_components(components: Vec<f64>) -> Self {
        Vector { components }
    }

    pub fn from_values(value: f64, size: usize) -> Self {
        Vector {
            components: vec![value; size],
        }
    }

    pub fn size(&self) -> usize {
        self.components.len()
    }

    pub fn get(&self, index: usize) -> f64 {
        self.components[index]
    }

    pub fn set(&mut self, index: usize, value: f64) {
        self.components[index] = value;
    }

    pub fn set_all(&mut self, value: f64) {
        for i in 0..self.size() {
            self.set(i, value);
        }
    }

    /// Returns the dot product of two vectors.
    /// If both vectors don't have the same size, the missing components are assumed to be 0.
    pub fn dot(&self, other: &Vector) -> f64 {
        let mut result = 0.0;
        let max_len = self.components.len().max(other.components.len());
        for i in 0..max_len {
            let a = self.components.get(i).unwrap_or(&0.0);
            let b = other.components.get(i).unwrap_or(&0.0);
            result += a * b;
        }
        result
    }

    /// Cross product of two 3-dimensions vectors
    pub fn cross(&self, other: &Vector) -> Vector {
        if self.components.len() != 3 || other.components.len() != 3 {
            panic!("Cross product is only defined for 3-dimensional vectors");
        }

        let a = &self.components;
        let b = &other.components;

        Vector::from_components(vec![
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ])
    }

    /// Transforms a vector V into V = V + aX.
    /// Both vectors are assumed to be of the same length.
    /// (similar to BLAS axpy).
    pub fn plus_x_times_a(&mut self, a: f64, x: &Vector) {
        for i in 0..self.size() {
            self.components[i] += a * x.components[i];
        }
    }

    /// Returns the index of the first element having the maximum absolute value.
    /// (similar to BLAS iamax).
    pub fn index_of_absolute_max(&self) -> usize {
        let mut absolute_maximum: f64 = self.components[0].abs();
        let mut max_index = 0;
        for i in 1..self.size() {
            if self.get(i).abs() > absolute_maximum {
                absolute_maximum = self.get(i).abs();
                max_index = i;
            }
        }

        max_index
    }

    /// Returns the euclidean norm of the vector.
    /// (similar to BLAS nrm2).
    pub fn norm(&self) -> f64 {
        if self.size() == 1 {
            return self.components[0].abs();
        }

        let mut x: f64;
        let mut abs_x: f64;
        let mut scale: f64 = 0.0;
        let mut scaled_square_sum: f64 = 1.0;

        for i in 0..self.size() {
            x = self.components[i];

            if x != 0.0 {
                abs_x = x.abs();

                if scale < abs_x {
                    scaled_square_sum = 1.0 + scaled_square_sum * (scale / abs_x).powi(2);
                    scale = abs_x;
                } else {
                    scaled_square_sum += (abs_x / scale).powi(2);
                }
            }
        }

        scale * scaled_square_sum.sqrt()
    }

    /// Returns the normalized vector
    pub fn normalized(&self) -> Self {
        let result: Vector = self.clone();
        result / self.norm()
    }

    /// Scale the vector by a constant.
    /// (similar to BLAS scal).
    pub fn scale(&mut self, a: f64) {
        for i in 0..self.size() {
            self.components[i] *= a;
        }
    }

    /// Interchanges two vectors (similar to BLAS swap)
    pub fn swap(&mut self, other: &mut Vector) {
        std::mem::swap(self, other);
    }

    /// Apply a given permutation to the vector
    pub fn apply_permutation(&mut self, permutation: Permutation) {
        let mut new_components: Vec<f64> = vec![0.0; self.size()];
        for (i, &j) in permutation.data().iter().enumerate() {
            new_components[i] = self.get(j);
        }
        self.components = new_components;
    }

    pub fn apply_function<F>(&mut self, f: F)
    where
        F: Fn(f64) -> f64,
    {
        for x in &mut self.components {
            *x = f(*x);
        }
    }

    pub fn is_almost_equal(&self, other: &Self, tolerance: f64) -> bool {
        if self.size() != other.size() {
            return false;
        }

        self.components
        .iter()
        .zip(other.components.iter())
        .all(|(a, b)| (a - b).abs() <= tolerance)
    }
}

/// Applies a Givens rotation of an angle -θ (in radians) on two vectors.
/// Both vectors must be of the same size.
/// (Similar to BLAS rot).
pub fn apply_givens_rotation(v1: &mut Vector, v2: &mut Vector, angle: f64) {
    if v1.size() != v2.size() {
        panic!("Vectors must have the same size to apply a Givens rotation");
    }

    let c: f64 = angle.cos();
    let s: f64 = angle.sin();

    let mut x: f64;
    let mut y: f64;

    for i in 0..v1.size() {
        x = v1.get(i);
        y = v2.get(i);
        v1.components[i] = c * x + s * y;
        v2.components[i] = -s * x + c * y;
    }
}

/// Returns the outer product of two vectors.
pub fn outer_product(v1: &Vector, v2: &Vector) -> Matrix {
    let n_row = v1.size();
    let n_col = v2.size();
    let mut result = Matrix::from_value(0.0, n_row, n_col);

    let mut matrix_index: usize;
    for row in 0..n_row {
        for column in 0..n_col {
            matrix_index = row * n_col + column;
            result.components[matrix_index] = v1.get(row) * v2.get(column);
        }
    }

    result
}

/// Vector + Vector.
/// If both vectors don't have the same size, the missing components are assumed to be 0.
impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                self.components.get(i).copied().unwrap_or(0.0)
                    + rhs.components.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Self {
            components: result_components,
        }
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                self.components.get(i).copied().unwrap_or(0.0)
                    + rhs.components.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Vector {
            components: result_components,
        }
    }
}

/// Vector + scalar
impl Add<f64> for Vector {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        let result_components = self.components.iter().map(|x| x + rhs).collect();
        Self {
            components: result_components,
        }
    }
}

impl Add<&f64> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &f64) -> Vector {
        let result_components = self.components.iter().map(|x| x + rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Add<&f64> for Vector {
    type Output = Vector;

    fn add(self, rhs: &f64) -> Vector {
        let result_components = self.components.iter().map(|x| x + rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Add<f64> for &Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Vector {
        let result_components = self.components.iter().map(|x| x + rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

/// Scalar + Vector
impl Add<Vector> for f64 {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        rhs + self
    }
}

impl Add<&Vector> for &f64 {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        rhs + self
    }
}

impl Add<Vector> for &f64 {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        rhs + self
    }
}

impl Add<&Vector> for f64 {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        rhs + self
    }
}

/// Vector - Vector
impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                self.components.get(i).copied().unwrap_or(0.0)
                    - rhs.components.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Self {
            components: result_components,
        }
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                self.components.get(i).copied().unwrap_or(0.0)
                    - rhs.components.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Vector {
            components: result_components,
        }
    }
}

impl Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                self.components.get(i).copied().unwrap_or(0.0)
                    - rhs.components.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Vector {
            components: result_components,
        }
    }
}

impl Sub<Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                self.components.get(i).copied().unwrap_or(0.0)
                    - rhs.components.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Vector {
            components: result_components,
        }
    }
}

/// Vector -= Vector
impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        for (i, &value) in rhs.components.iter().enumerate() {
        if i < self.components.len() {
            self.components[i] -= value;
        } else {
            self.components.push(-value);
        }
    }
    }
}

/// Vector - scalar
impl Sub<f64> for Vector {
    type Output = Self;

    fn sub(self, rhs: f64) -> Vector {
        let result_components = self.components.iter().map(|x| x - rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Sub<&f64> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &f64) -> Vector {
        let result_components = self.components.iter().map(|x| x - rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Sub<&f64> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &f64) -> Vector {
        let result_components = self.components.iter().map(|x| x - rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Sub<f64> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: f64) -> Vector {
        let result_components = self.components.iter().map(|x| x - rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

/// scalar - vector
impl Sub<Vector> for f64 {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        -rhs + self
    }
}

impl Sub<&Vector> for &f64 {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        -rhs + self
    }
}

impl Sub<&Vector> for f64 {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        -rhs + self
    }
}

impl Sub<Vector> for &f64 {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        -rhs + self
    }
}

/// Vector * Vector.
/// Component-wise product of two vectors.
/// If both vectors don't have the same length, the missing components are assumed to be 0.
impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                self.components.get(i).copied().unwrap_or(0.0)
                    * rhs.components.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Self {
            components: result_components,
        }
    }
}

impl Mul for &Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Vector {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                self.components.get(i).copied().unwrap_or(0.0)
                    * rhs.components.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Vector {
            components: result_components,
        }
    }
}

/// Vector * scalar
impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let result_components = self.components.iter().map(|x| x * rhs).collect();
        Self {
            components: result_components,
        }
    }
}

impl Mul<&f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: &f64) -> Vector {
        let result_components = self.components.iter().map(|x| x * rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Mul<&f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: &f64) -> Vector {
        let result_components = self.components.iter().map(|x| x * rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        let result_components = self.components.iter().map(|x| x * rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

/// scalar * Vector
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        rhs * self
    }
}

impl Mul<&Vector> for &f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        rhs * self
    }
}

impl Mul<Vector> for &f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        rhs * self
    }
}

impl Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        rhs * self
    }
}

/// Vector / Vector.
/// Component-wise division of two vectors.
/// If both vectors don't have the same length, the missing components are assumed to be 0.
impl Div for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                let left = self.components.get(i).copied().unwrap_or(0.0);
                let right = rhs.components.get(i).copied().unwrap_or(0.0);
                if right == 0.0 { 0.0 } else { left / right }
            })
            .collect();

        Self {
            components: result_components,
        }
    }
}

impl Div for &Vector {
    type Output = Vector;

    fn div(self, rhs: Self) -> Vector {
        let n = self.components.len().max(rhs.components.len());

        let result_components = (0..n)
            .map(|i| {
                let left = self.components.get(i).copied().unwrap_or(0.0);
                let right = rhs.components.get(i).copied().unwrap_or(0.0);
                if right == 0.0 { 0.0 } else { left / right }
            })
            .collect();

        Vector {
            components: result_components,
        }
    }
}

/// Vector / scalar
impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let result_components = self.components.iter().map(|x| x / rhs).collect();
        Self {
            components: result_components,
        }
    }
}

impl Div<&f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: &f64) -> Vector {
        let result_components = self.components.iter().map(|x| x / rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        let result_components = self.components.iter().map(|x| x / rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

impl Div<&f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: &f64) -> Vector {
        let result_components = self.components.iter().map(|x| x / rhs).collect();
        Vector {
            components: result_components,
        }
    }
}

/// Vector /= scalar
impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.components = self.components.iter().map(|x| x / rhs).collect();
    }
}

/// scalar / Vector
impl Div<Vector> for f64 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Vector {
        let n = rhs.components.len();
        let result_components = (0..n)
            .map(|i| {
                let right = rhs.components.get(i).copied().unwrap_or(0.0);
                if right == 0.0 { 0.0 } else { self / right }
            })
            .collect();
        Vector {
            components: result_components,
        }
    }
}

impl Div<&Vector> for &f64 {
    type Output = Vector;

    fn div(self, rhs: &Vector) -> Vector {
        let n = rhs.components.len();
        let result_components = (0..n)
            .map(|i| {
                let right = rhs.components.get(i).copied().unwrap_or(0.0);
                if right == 0.0 { 0.0 } else { self / right }
            })
            .collect();
        Vector {
            components: result_components,
        }
    }
}

impl Div<Vector> for &f64 {
    type Output = Vector;

    fn div(self, rhs: Vector) -> Vector {
        let n = rhs.components.len();
        let result_components = (0..n)
            .map(|i| {
                let right = rhs.components.get(i).copied().unwrap_or(0.0);
                if right == 0.0 { 0.0 } else { self / right }
            })
            .collect();
        Vector {
            components: result_components,
        }
    }
}

impl Div<&Vector> for f64 {
    type Output = Vector;

    fn div(self, rhs: &Vector) -> Vector {
        let n = rhs.components.len();
        let result_components = (0..n)
            .map(|i| {
                let right = rhs.components.get(i).copied().unwrap_or(0.0);
                if right == 0.0 { 0.0 } else { self / right }
            })
            .collect();
        Vector {
            components: result_components,
        }
    }
}

/// -vector
impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            components: self.components.iter().map(|x| -x).collect(),
        }
    }
}

impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            components: self.components.iter().map(|x| -x).collect(),
        }
    }
}

/// Pretty print a vector in the form v = [1.0, 2.0, 3.0]
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, component) in self.components.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            if component >= &0.0 {
                write!(f, " ")?;
            }

            write!(f, "{:.8}", component)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
