use std::{
    fmt, ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{linear_algebra::{Vector, solve_for_matrix}, permutation::Permutation};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub components: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        if rows == 0 || cols == 0 {
            panic!("Incompatible matrix dimensions");
        }
        
        Matrix::from_value(0.0, rows, cols)
    }

    pub fn from_components(components: Vec<f64>, rows: usize, cols: usize) -> Self {
        if rows == 0 || cols == 0 || components.len() != rows * cols {
            panic!("Incompatible matrix dimensions");
        }
        Matrix {
            components,
            rows,
            cols,
        }
    }

    pub fn from_value(value: f64, rows: usize, cols: usize) -> Self {
        let components = vec![value; rows * cols];
        Matrix::from_components(components, rows, cols)
    }

    /// Returns a new matrix with random components from -1 to 1
    pub fn random(rows: usize, cols: usize) -> Self {
        let components: Vec<f64> = (0..rows * cols)
            .map(|_| rand::random::<f64>() * 2.0 - 1.0)
            .collect();
        Matrix::from_components(components, rows, cols)
    }

    /// Returnst the NxN identity matrix
    pub fn identity(size: usize) -> Self {
        let mut result: Matrix = Matrix::from_value(0.0, size, size);
        for i in 0..size {
            result.set(i, i, 1.0);
        }
        result
    }

    /// Returns the element of the matrix at the specified row and column.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds");
        }
        self.components[row * self.cols + col]
    }

    /// Returns a specific column of the matrix
    pub fn get_column(&self, col: usize) -> Vec<f64> {
        if col >= self.cols {
            panic!("Column index out of bounds");
        }
        (0..self.rows).map(|row| self.get(row, col)).collect()
    }

    /// Returns a specific column of the matrix in the Vector form
    pub fn get_column_vector(&self, col: usize) -> Vector {
        if col >= self.cols {
            panic!("Column index out of bounds");
        }
        let components: Vec<f64> = (0..self.rows).map(|row| self.get(row, col)).collect();
        Vector::from_components(components)
    }

    /// Returns a specific row of the matrix
    pub fn get_row(&self, row: usize) -> Vec<f64> {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        (0..self.cols).map(|col| self.get(row, col)).collect()
    }

    /// Returns a specific row of the matrix in the Vector form
    pub fn get_row_vector(&self, row: usize) -> Vector {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        let components: Vec<f64> = (0..self.cols).map(|col| self.get(row, col)).collect();
        Vector::from_components(components)
    }

    /// Returns the diagonal elements of self
    pub fn get_diagonal_elements(&self) -> Vec<f64> {
        let mut result: Vec<f64> = vec![];
        let n: usize = self.cols.min(self.rows);
        for i in 0..n {
            result.push(self.get(i, i));
        }
        result
    }

    /// Sets a specific element of the matrix.
    /// The transformation is made in-place.
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds");
        }
        self.components[row * self.cols + col] = value;
    }

    /// Sets a specific column of the matrix.
    /// The transformation is made in-place
    pub fn set_column(&mut self, column_index: usize, new_column: Vec<f64>) {
        if new_column.len() > self.rows || column_index > self.cols {
            panic!("Index out of bound!");
        }

        for (index, element) in new_column.iter().enumerate() {
            self.set(index, column_index, *element);
        }
    }

    /// Swaps two columns of the matrix.
    /// The modification is made in-place.
    pub fn swap_columns(&mut self, col1: usize, col2: usize) {
        if col1 >= self.cols || col2 >= self.cols {
            panic!("Column index out of bounds");
        }
        for row in 0..self.rows {
            let temp = self.get(row, col1);
            *self.components.get_mut(row * self.cols + col1).unwrap() = self.get(row, col2);
            *self.components.get_mut(row * self.cols + col2).unwrap() = temp;
        }
    }

    /// Returns the matrix with two columns swapped
    pub fn swapped_columns(&self, col1: usize, col2: usize) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.swap_columns(col1, col2);
        new_matrix
    }

    /// Swaps two rows of the matrix
    /// The modification is made in-place
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        if row1 >= self.rows || row2 >= self.rows {
            panic!("Row index out of bounds");
        }
        for column in 0..self.cols {
            self.components
                .swap(row1 * self.cols + column, row2 * self.cols + column);
        }
    }

    /// Returns the matrix with two rows swapped
    pub fn swapped_rows(&self, row1: usize, row2: usize) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.swap_rows(row1, row2);
        new_matrix
    }

    // Apply row permutations as provided by a permutation vector
    pub fn apply_row_permutation(&self, permutations: Permutation) -> Matrix {
        let mut result = Matrix::from_value(0.0, self.rows, self.cols);

        for (i, &j) in permutations.data().iter().enumerate() {
            for col in 0..self.cols {
                result.set(i, col, self.get(j, col));
            }
        }

        result
    }

    /// Adds a scalar to a specific column.
    /// The modification is made in-place
    pub fn add_to_column(&mut self, col: usize, scalar: f64) {
        if col >= self.cols {
            panic!("Column index out of bounds");
        }
        for row in 0..self.rows {
            *self.components.get_mut(row * self.cols + col).unwrap() += scalar;
        }
    }

    /// Adds a scalar to a specific column and returns the resulting matrix
    pub fn added_to_column(&self, col: usize, scalar: f64) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.add_to_column(col, scalar);
        new_matrix
    }

    ///Adds a scalar to a specific row.
    /// The modification is made in-place.
    pub fn add_to_row(&mut self, row: usize, scalar: f64) {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        for col in 0..self.cols {
            *self.components.get_mut(row * self.cols + col).unwrap() += scalar;
        }
    }

    /// Adds a scalar to a specific row and returns the resulting matrix
    pub fn added_to_row(&self, row: usize, scalar: f64) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.add_to_row(row, scalar);
        new_matrix
    }

    /// Adds a multiple of one row to another row.
    /// The modification is made in-place.
    pub fn add_multiple_of_row(&mut self, target_row: usize, source_row: usize, scalar: f64) {
        if target_row >= self.rows || source_row >= self.rows {
            panic!("Row index out of bounds");
        }
        for col in 0..self.cols {
            *self
                .components
                .get_mut(target_row * self.cols + col)
                .unwrap() += scalar * self.get(source_row, col);
        }
    }

    /// Subtracts a scalar from a specific column.
    /// The modification is made in-place.
    pub fn subtract_from_column(&mut self, col: usize, scalar: f64) {
        if col >= self.cols {
            panic!("Column index out of bounds");
        }
        for row in 0..self.rows {
            *self.components.get_mut(row * self.cols + col).unwrap() -= scalar;
        }
    }

    /// Subtracts a scalar from a specific column and returns the resulting matrix
    pub fn subtracted_from_column(&self, col: usize, scalar: f64) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.subtract_from_column(col, scalar);
        new_matrix
    }

    /// Subtracts a scalar from a specific row.
    /// The modification is made in-place.
    pub fn subtract_from_row(&mut self, row: usize, scalar: f64) {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        for col in 0..self.cols {
            *self.components.get_mut(row * self.cols + col).unwrap() -= scalar;
        }
    }

    /// Subtracts a scalar from a specific row and returns the resulting matrix
    pub fn subtracted_from_row(&self, row: usize, scalar: f64) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.subtract_from_row(row, scalar);
        new_matrix
    }

    /// Multiply a specific column by a scalar
    /// The modification is made in-place
    pub fn multiply_column(&mut self, col: usize, scalar: f64) {
        if col >= self.cols {
            panic!("Column index out of bounds");
        }
        for row in 0..self.rows {
            *self.components.get_mut(row * self.cols + col).unwrap() *= scalar;
        }
    }

    /// Multiply a specific column by a scalar and returns the resulting matrix
    pub fn multiplied_column(&self, col: usize, scalar: f64) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.multiply_column(col, scalar);
        new_matrix
    }

    /// Multiply a specific row by a scalar
    /// The modification is made in-place
    pub fn multiply_row(&mut self, row: usize, scalar: f64) {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        for col in 0..self.cols {
            *self.components.get_mut(row * self.cols + col).unwrap() *= scalar;
        }
    }

    /// Multiply a specific row by a scalar and returns the resulting matrix
    pub fn multiplied_row(&self, row: usize, scalar: f64) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.multiply_row(row, scalar);
        new_matrix
    }

    /// Divide a specific column by a scalar
    /// The modification is made in-place
    pub fn divide_column(&mut self, col: usize, scalar: f64) {
        if col >= self.cols {
            panic!("Column index out of bounds");
        }
        for row in 0..self.rows {
            *self.components.get_mut(row * self.cols + col).unwrap() /= scalar;
        }
    }

    /// Divide a specific column by a scalar and returns the resulting matrix
    pub fn divided_column(&self, col: usize, scalar: f64) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.divide_column(col, scalar);
        new_matrix
    }

    /// Divide a specific row by a scalar
    /// The modification is made in-place
    pub fn divide_row(&mut self, row: usize, scalar: f64) {
        if row >= self.rows {
            panic!("Row index out of bounds");
        }
        for col in 0..self.cols {
            *self.components.get_mut(row * self.cols + col).unwrap() /= scalar;
        }
    }

    /// Divide a specific row by a scalar and returns the resulting matrix
    pub fn divided_row(&self, row: usize, scalar: f64) -> Self {
        let mut new_matrix = self.clone();
        new_matrix.divide_row(row, scalar);
        new_matrix
    }

    /// Applies the following rank-1 update to the matrix : A = A + α * outer_product(x, y).
    pub fn general_rank_1_update(&mut self, alpha: f64, x: &Vector, y: &Vector) {
        let mut tmp: f64;
        for row in 0..self.rows {
            tmp = alpha * x.get(row);
            for col in 0..self.cols {
                self.components[row * self.cols + col] += y.get(col) * tmp;
            }
        }
    }

    /// Returns the trace of the matrix
    pub fn trace(&self) -> f64 {
        let mut result: f64 = 0.0;
        let n: usize = self.cols.min(self.rows);
        for i in 0..n {
            result += self.get(i, i);
        }
        result
    }

    /// Returns the rank of the matrix using Gaussian elimination.
    pub fn rank(&self) -> usize {
        let mut matrix = self.clone();

        let rows = matrix.rows;
        let cols = matrix.cols;

        let mut rank = 0;
        let mut pivot_row = 0;

        let tolerance = 1e-10;

        for col in 0..cols {
            if pivot_row >= rows {
                break;
            }

            // Find the row with the largest absolute value in this column.
            let mut max_row = pivot_row;

            for row in (pivot_row + 1)..rows {
                if matrix.get(row, col).abs()
                    > matrix.get(max_row, col).abs()
                {
                    max_row = row;
                }
            }

            // Skip this column if the pivot is approximately zero.
            if matrix.get(max_row, col).abs() < tolerance {
                continue;
            }

            // Swap the pivot row into position.
            if max_row != pivot_row {
                for j in 0..cols {
                    let temp = matrix.get(pivot_row, j);
                    matrix.set(
                        pivot_row,
                        j,
                        matrix.get(max_row, j),
                    );
                    matrix.set(max_row, j, temp);
                }
            }

            // Eliminate entries below the pivot.
            let pivot = matrix.get(pivot_row, col);

            for row in (pivot_row + 1)..rows {
                let factor = matrix.get(row, col) / pivot;

                for j in col..cols {
                    let value = matrix.get(row, j)
                        - factor * matrix.get(pivot_row, j);

                    matrix.set(row, j, value);
                }
            }

            rank += 1;
            pivot_row += 1;
        }

        rank
    }

    pub fn off_diagonal_norm(&self) -> f64 {
        let mut sum = 0.0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if i != j {
                    let value = self.get(i, j);
                    sum += value * value;
                }
            }
        }

        sum.sqrt()
    }

    /// Returns the eigenvectors and the eigenvalues of self
    pub fn eigenvectors(&self, tolerance: f64) -> (Vec<Vector>, Vec<f64>) {
        let mut eigenvalues: Matrix = self.clone();
        let mut eigenvectors: Matrix = Matrix::identity(self.cols);
        let mut continue_loop: bool = true;
        while continue_loop {
            let temp: f64 = eigenvalues.get(0, 0);
            let (q, r) = eigenvalues.qr_decomposition();
            eigenvalues = &r * &q;
            eigenvectors = eigenvectors * &q;

            // Testing for convergence
            if eigenvalues.off_diagonal_norm() < tolerance {
                continue_loop = false;
            }
        }

        // Turning the eigenvectors matrix into a list of vectors
        let mut eigenvector_list: Vec<Vector> = Vec::new();
        for column in 0..eigenvectors.cols {
            eigenvector_list.push(eigenvectors.get_column_vector(column));
        }

        (eigenvector_list, eigenvalues.get_diagonal_elements())
    }

    /// Returns just the eigenvalues of self
    pub fn eigenvalues(&self, tolerance: f64) -> Vec<f64> {
        let mut eigenvalues: Matrix = self.clone();
        let mut continue_loop: bool = true;
        while continue_loop {
            let (q, r) = eigenvalues.qr_decomposition();
            eigenvalues = &r * &q;

            // Testing for convergence
            if eigenvalues.off_diagonal_norm() < tolerance {
                continue_loop = false;
            }
        }

        eigenvalues.get_diagonal_elements()
    }

    pub fn determinant(&self) -> f64 {
        let (l, u, p) = self.lu_decomposition();
        let n: usize = self.cols;
        let mut result: f64 = 1.0;
        for i in 0..n {
            result *= l.get(i, i) * u.get(i, i);
        }

        result *= (-1_i32).pow(p.minimum_swap()) as f64;

        result
    }

    pub fn inverse(&self) -> Result<Matrix, String> {
        solve_for_matrix(self, &Matrix::identity(self.cols))
    }
}

/// Addition of two matrices
impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Incompatible matrix dimensions for addition!");
        }
        let components = self
            .components
            .iter()
            .zip(rhs.components.iter())
            .map(|(a, b)| a + b)
            .collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Matrix {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Incompatible matrix dimensions for addition!");
        }
        let components = self
            .components
            .iter()
            .zip(rhs.components.iter())
            .map(|(a, b)| a + b)
            .collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

/// Matrix + scalar
impl Add<f64> for Matrix {
    type Output = Self;

    fn add(self, scalar: f64) -> Self {
        let components = self.components.iter().map(|x| x + scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Add<&f64> for &Matrix {
    type Output = Matrix;

    fn add(self, scalar: &f64) -> Matrix {
        let components = self.components.iter().map(|x| x + scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Add<&f64> for Matrix {
    type Output = Matrix;

    fn add(self, scalar: &f64) -> Matrix {
        let components = self.components.iter().map(|x| x + scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Add<f64> for &Matrix {
    type Output = Matrix;

    fn add(self, scalar: f64) -> Matrix {
        let components = self.components.iter().map(|x| x + scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

/// scalar + matrix
impl Add<Matrix> for f64 {
    type Output = Matrix;

    fn add(self, matrix: Matrix) -> Matrix {
        matrix + self
    }
}

impl Add<&Matrix> for &f64 {
    type Output = Matrix;

    fn add(self, matrix: &Matrix) -> Matrix {
        matrix + self
    }
}

impl Add<&Matrix> for f64 {
    type Output = Matrix;

    fn add(self, matrix: &Matrix) -> Matrix {
        matrix + self
    }
}

impl Add<Matrix> for &f64 {
    type Output = Matrix;

    fn add(self, matrix: Matrix) -> Matrix {
        matrix + self
    }
}

/// Matrix - Matrix
impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Matrix {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Incompatible matrix dimensions for subtraction!");
        }
        let components = self
            .components
            .iter()
            .zip(rhs.components.iter())
            .map(|(a, b)| a - b)
            .collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Sub for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Matrix {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Incompatible matrix dimensions for subtraction!");
        }
        let components = self
            .components
            .iter()
            .zip(rhs.components.iter())
            .map(|(a, b)| a - b)
            .collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

/// Matrix - scalar
impl Sub<f64> for Matrix {
    type Output = Self;

    fn sub(self, scalar: f64) -> Self {
        let components = self.components.iter().map(|x| x - scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Sub<&f64> for &Matrix {
    type Output = Matrix;

    fn sub(self, scalar: &f64) -> Matrix {
        let components = self.components.iter().map(|x| x - scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Sub<&f64> for Matrix {
    type Output = Matrix;

    fn sub(self, scalar: &f64) -> Matrix {
        let components = self.components.iter().map(|x| x - scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Sub<f64> for &Matrix {
    type Output = Matrix;

    fn sub(self, scalar: f64) -> Matrix {
        let components = self.components.iter().map(|x| x - scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

/// scalar - Matrix
impl Sub<Matrix> for f64 {
    type Output = Matrix;

    fn sub(self, matrix: Matrix) -> Matrix {
        let components = matrix.components.iter().map(|x| self - x).collect();
        Matrix {
            components,
            rows: matrix.rows,
            cols: matrix.cols,
        }
    }
}

impl Sub<&Matrix> for &f64 {
    type Output = Matrix;

    fn sub(self, matrix: &Matrix) -> Matrix {
        let components = matrix.components.iter().map(|x| self - x).collect();
        Matrix {
            components,
            rows: matrix.rows,
            cols: matrix.cols,
        }
    }
}

impl Sub<Matrix> for &f64 {
    type Output = Matrix;

    fn sub(self, matrix: Matrix) -> Matrix {
        let components = matrix.components.iter().map(|x| self - x).collect();
        Matrix {
            components,
            rows: matrix.rows,
            cols: matrix.cols,
        }
    }
}

impl Sub<&Matrix> for f64 {
    type Output = Matrix;

    fn sub(self, matrix: &Matrix) -> Matrix {
        let components = matrix.components.iter().map(|x| self - x).collect();
        Matrix {
            components,
            rows: matrix.rows,
            cols: matrix.cols,
        }
    }
}

/// Matrix * Matrix
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self.cols != rhs.rows {
            panic!("Incompatible matrix dimensions for multiplication!");
        }

        let mut new_components: Vec<f64> = vec![0.0; self.rows * rhs.cols];

        for i in 0..self.rows {
            for k in 0..self.cols {
                let aik = self.get(i, k);

                for j in 0..rhs.cols {
                    new_components[i * rhs.cols + j] += aik * rhs.get(k, j);
                }
            }
        }
        Matrix {
            components: new_components,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Matrix {
        if self.cols != rhs.rows {
            panic!("Incompatible matrix dimensions for multiplication!");
        }

        let mut new_components: Vec<f64> = vec![0.0; self.rows * rhs.cols];

        let mut sum: f64;
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                sum = 0.0;

                for k in 0..self.cols {
                    sum += self.get(i, k) * rhs.get(k, j);
                }

                new_components[i * rhs.cols + j] = sum;
            }
        }
        Matrix {
            components: new_components,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: &Matrix) -> Self {
        if self.cols != rhs.rows {
            panic!("Incompatible matrix dimensions for multiplication!");
        }

        let mut new_components: Vec<f64> = vec![0.0; self.rows * rhs.cols];

        for i in 0..self.rows {
            for k in 0..self.cols {
                let aik = self.get(i, k);

                for j in 0..rhs.cols {
                    new_components[i * rhs.cols + j] += aik * rhs.get(k, j);
                }
            }
        }
        Matrix {
            components: new_components,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        if self.cols != rhs.rows {
            panic!("Incompatible matrix dimensions for multiplication!");
        }

        let mut new_components: Vec<f64> = vec![0.0; self.rows * rhs.cols];

        for i in 0..self.rows {
            for k in 0..self.cols {
                let aik = self.get(i, k);

                for j in 0..rhs.cols {
                    new_components[i * rhs.cols + j] += aik * rhs.get(k, j);
                }
            }
        }
        Matrix {
            components: new_components,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

/// Matrix * scalar
impl Mul<f64> for Matrix {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        let components = self.components.iter().map(|x| x * scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Mul<&f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, scalar: &f64) -> Matrix {
        let components = self.components.iter().map(|x| x * scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, scalar: f64) -> Matrix {
        let components = self.components.iter().map(|x| x * scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Mul<&f64> for Matrix {
    type Output = Matrix;

    fn mul(self, scalar: &f64) -> Matrix {
        let components = self.components.iter().map(|x| x * scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

/// scalar * Matrix
impl Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, matrix: Matrix) -> Self::Output {
        matrix * self
    }
}

impl Mul<&Matrix> for &f64 {
    type Output = Matrix;

    fn mul(self, matrix: &Matrix) -> Self::Output {
        matrix * self
    }
}

impl Mul<Matrix> for &f64 {
    type Output = Matrix;

    fn mul(self, matrix: Matrix) -> Self::Output {
        matrix * self
    }
}

impl Mul<&Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, matrix: &Matrix) -> Self::Output {
        matrix * self
    }
}

/// Matrix / scalar
impl Div<f64> for Matrix {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        let components = self.components.iter().map(|x| x / scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Div<&f64> for &Matrix {
    type Output = Matrix;

    fn div(self, scalar: &f64) -> Matrix {
        let components = self.components.iter().map(|x| x / scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Div<f64> for &Matrix {
    type Output = Matrix;

    fn div(self, scalar: f64) -> Matrix {
        let components = self.components.iter().map(|x| x / scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Div<&f64> for Matrix {
    type Output = Matrix;

    fn div(self, scalar: &f64) -> Matrix {
        let components = self.components.iter().map(|x| x / scalar).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

/// -Matrix
impl Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self {
        let components = self.components.iter().map(|x| -x).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl Neg for &Matrix {
    type Output = Matrix;

    fn neg(self) -> Matrix {
        let components = self.components.iter().map(|x| -x).collect();
        Matrix {
            components,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

/// Pretty printing a matrix
/// in the form [1, 2, 3]
///             |4, 5, 6|
///             [7, 8, 9]
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            write!(f, "[ ")?;

            for col in 0..self.cols {
                let component: f64 = self.components[row * self.cols + col];

                if component >= 0.0 {
                    write!(f, " ")?;
                }

                write!(f, "{:.8}", component).unwrap();

                if col < self.cols - 1 {
                    write!(f, " ")?;
                }
            }

            writeln!(f, " ]")?;
        }
        Ok(())
    }
}
