use crate::linear_algebra::*;

impl Matrix {
    /// Returns the decomposition of self as the product of an orthogonal matrix Q and an upper-triangular matrix R.
    pub fn qr_decomposition(&self) -> (Matrix, Matrix) {
        let n: usize = self.cols;

        let mut u_n: Vec<Vector> = vec![Vector::from_components(self.get_column(0))];
        
        let mut q: Matrix = Matrix::from_value(0.0, n, n);
        q.set_column(0, u_n[0].normalized().components);

        for i in 1..n {
            let a_i = self.get_column_vector(i);
            let mut u_i: Vector = a_i.clone();
            
            for j in 0..i {
                u_i -= (q.get_column_vector(j).dot(&a_i)) * q.get_column_vector(j);
            }

            u_n.push(u_i);
            q.set_column(i,u_n[i].normalized().components);
        }

        let mut r: Matrix = Matrix::new(n, n);

        for i in 0..n {
            r.set(i, i, u_n[i].norm());
        }

        for row in 0..(n-1) {
            for col in (row+1)..n {
                r.set(row, col, q.get_column_vector(row).dot(&self.get_column_vector(col)));
            }
        }

        (q, r)
    }
}