use crate::{linear_algebra::*, permutation::Permutation};

pub struct LUDecomposition {
    pub l: Matrix,
    pub u: Matrix,
    pub p: Permutation,
}

impl Matrix {
    /// This function performs the LU decomposition of a square matrix.
    /// If successful, it returns a LUDecomposition type with the L and U matrices as well as a permutation vector P.
    pub fn lu_decomposition(&self) -> (Matrix, Matrix, Permutation) {
        let n_cols: usize = self.cols;
        let n_rows: usize = self.rows;
        let n_iterations: usize = usize::min(n_rows, n_cols);

        let mut result = self.clone();
        let mut l = Matrix::from_value(0.0, n_rows, n_cols);
        let mut permutations:Permutation = Permutation::new(n_rows);

        for current_row in 0..n_iterations {
            // Applying partial pivoting
            let mut max_column_index = current_row;
            let mut max_column_value: f64 = result.get(current_row, current_row);
            for i in (current_row + 1)..n_rows {
                let current_value: f64 = result.get(i, current_row);
                if current_value.abs() > max_column_value.abs() {
                    max_column_index = i;
                    max_column_value = current_value;
                }
            }

            result.swap_rows(current_row, max_column_index);
            permutations.swap(current_row, max_column_index);
            l.swap_rows(current_row, max_column_index);

            // Updating L
            for i in current_row..n_iterations {
                l.set(i, current_row, result.get(i, current_row));
            }

            // We divide the current row by its diagonal element
            let diagonal_element: f64 = result.get(current_row, current_row);
            if diagonal_element != 0.0 {
                result.divide_row(current_row, diagonal_element);
            }

            // Then we substract the first row a proper amount of time from the other rows
            for next_row in (current_row + 1)..n_iterations {
                let multiplier: f64 = result.get(next_row, current_row);
                if multiplier != 0.0 {
                    result.add_multiple_of_row(next_row, current_row, -multiplier);
                }
            }
        }

        (l, result, permutations)
    }
}
