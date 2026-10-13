use crate::linear_algebra::{Matrix, Vector};

/// This function solves the system of linear equations Ax = b using LU decomposition
pub fn solve_for_vector(a: &Matrix, v: &Vector) -> Result<Vector, String> {
    let (l, u, p) = a.lu_decomposition();

    let mut v_with_permutation = v.clone();
    v_with_permutation.apply_permutation(p);

    // Forward substitution: Ly = Pb
    let mut y = Vector::from_values(0.0, a.rows);

    for row in 0..l.rows {
        let mut sum = 0.0;

        for j in 0..row {
            sum += l.get(row, j) * y.get(j);
        }

        y.set(
            row,
            (v_with_permutation.get(row) - sum) / l.get(row, row),
        );
    }

    // Backward substitution: Ux = y
    let mut result = Vector::from_values(0.0, a.rows);

    for row in (0..u.rows).rev() {
        let mut sum = 0.0;

        for j in (row + 1)..u.cols {
            sum += u.get(row, j) * result.get(j);
        }

        result.set(row, (y.get(row) - sum) / u.get(row, row));
    }

    Ok(result)
}

pub fn solve_for_matrix(a: &Matrix, v: &Matrix) -> Result<Matrix, String> {
    let mut result: Matrix = Matrix::new(v.rows, v.cols);
    for column in 0..(v.cols) {
        let new_column: Vector = solve_for_vector(a, &v.get_column_vector(column))?;
        result.set_column(column, new_column.components);
    }

    Ok(result)
}