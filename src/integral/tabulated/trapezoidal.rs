pub fn trapezoidal(xs: Vec<f64>, ys: Vec<f64>) -> f64 {
    // If the x axis is not sorted, we sort it with the y axis and recall the function with the sorted data
    if !xs.is_sorted() {
        let mut data: Vec<(f64, f64)> = xs.iter().copied().zip(ys.iter().copied()).collect();

        data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let (x_sorted, y_sorted): (Vec<f64>, Vec<f64>) = data.into_iter().unzip();

        return trapezoidal(x_sorted, y_sorted);
    }

    // If both axis don't have the same size we only integrate what we can
    let n: usize = xs.len().min(ys.len());
    let mut result: f64 = 0.0;

    for k in 1..n {
        let h: f64 = xs[k] - xs[k - 1];
        result += h * (ys[k] + ys[k - 1]) / 2.0;
    }

    result
}
