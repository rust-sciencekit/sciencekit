use crate::complex::*;
use crate::linear_algebra::Matrix;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub enum PolynomialError {
    InvalidPointsForInterpolation,
}

#[macro_export]
macro_rules! polynomial {
    ( $( $x:expr ),* ) => {
        {
            let mut coefficients: Vec<f64> = Vec::new();

            $(
                coefficients.push($x as f64);
            )*

            if coefficients.is_empty() {
                coefficients.push(0.0);
            }

            while coefficients.last() == Some(&0.0) && coefficients.len() > 1 {
                coefficients.pop();
            }

            Polynomial::from_coefficients(coefficients)
        }
    };
}

impl Default for Polynomial {
    fn default() -> Self {
        Self::new()
    }
}

impl Polynomial {
    /// Initialization of an empty polynomial.
    pub fn new() -> Self {
        Self {
            coefficients: vec![0.0],
        }
    }

    pub fn from_coefficients(coefficients: Vec<f64>) -> Self {
        if coefficients.is_empty() {
            Self {
                coefficients: vec![0.0],
            }
        } else {
            Self { coefficients }
        }
    }

    pub fn from_points(points: Vec<(f64, f64)>) -> Result<Self, PolynomialError> {
        if points.is_empty() {
            return Err(PolynomialError::InvalidPointsForInterpolation);
        }

        if points.iter().any(|(x, y)| !x.is_finite() || !y.is_finite()) {
            return Err(PolynomialError::InvalidPointsForInterpolation);
        }

        let n = points.len();

        // Extract x and y coordinates.
        let xs: Vec<f64> = points.iter().map(|(x, _)| *x).collect();
        let mut ys: Vec<f64> = points.iter().map(|(_, y)| *y).collect();

        // Compute Newton's divided differences in place.
        for j in 1..n {
            for i in (j..n).rev() {
                let denominator = xs[i] - xs[i - j];

                if denominator == 0.0 {
                    return Err(PolynomialError::InvalidPointsForInterpolation);
                }

                ys[i] = (ys[i] - ys[i - 1]) / denominator;
            }
        }

        // Convert the Newton representation into ascending-power
        // monomial coefficients.
        let mut coefficients = vec![ys[n - 1]];

        for i in (0..n - 1).rev() {
            let mut next = vec![0.0; coefficients.len() + 1];

            for (j, &coefficient) in coefficients.iter().enumerate() {
                next[j] -= xs[i] * coefficient;
                next[j + 1] += coefficient;
            }

            next[0] += ys[i];
            coefficients = next;
        }

        Ok(Self { coefficients })
    }

    pub fn from_roots(roots: Vec<f64>) -> Self {
        let mut coefficients = vec![1.0];

        for root in roots {
            let mut next = vec![0.0; coefficients.len() + 1];

            for (i, &coefficient) in coefficients.iter().enumerate() {
                next[i] -= root * coefficient;
                next[i + 1] += coefficient;
            }

            coefficients = next;
        }

        Self { coefficients }
    }

    /// Evaluates the polynomial for a given x ∈ ℝ.
    pub fn evaluate(&self, x: f64) -> f64 {
        let mut result: f64 = 0.0;

        for &coeff in self.coefficients.iter().rev() {
            result = result * x + coeff;
        }

        result
    }

    /// Evaluates the polynomial for a given x ∈ ℂ
    pub fn evaluate_complex(&self, at: Complex) -> Complex {
        let x: Complex = at;
        self.coefficients
            .iter()
            .rfold(crate::constants::complex::ZERO, |acc, &coeff| {
                acc * x + Complex::from_cartesian(coeff, 0.0)
            })
    }

    /// The degree of the polynomial.
    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    /// Returns the companion matrix of the polynomial
    pub fn companion_matrix(&self) -> Matrix {
        let n: usize = self.degree();
        let mut companion_matrix: Matrix = Matrix::from_value(0.0, n, n);
        
        // Updating the bottom line coefficients
        for i in 0..n {
            companion_matrix.set(n-1, i, -self.coefficients[i]);
        }

        // Updating the diagonal elements
        let a_n: f64 = *self.coefficients.last().unwrap();
        for i in 0..(n-1) {
            companion_matrix.set(i, i+1, a_n);
        }

        companion_matrix / a_n
    }

    /// Transforms the polynomial into its derivative
    pub fn differentiate(&mut self) {
        let n: usize = self.coefficients.len();

        if n <= 1 {
            self.coefficients.clear();
            self.coefficients.push(0.0);
            return;
        }

        for i in 1..n {
            self.coefficients[i - 1] = self.coefficients[i] * i as f64;
        }

        self.coefficients.pop();
    }

    /// Returns the derivative of the polynomial
    pub fn derivative(&self) -> Self {
        let mut result = self.clone();
        result.differentiate();
        result
    }

    /// Transforms the polynomial into its definite integral
    pub fn integrate(&mut self, constant: f64) {
        let n = self.coefficients.len();

        // add space for new highest power term
        self.coefficients.resize(n + 1, 0.0);

        // shift and divide backwards to avoid overwriting
        for i in (0..n).rev() {
            self.coefficients[i + 1] = self.coefficients[i] / (i as f64 + 1.0);
        }

        // set integration constant
        self.coefficients[0] = constant;
    }

    /// Returns the integral of the polynomial
    pub fn integral(&self, constant: f64) -> Polynomial {
        let mut result: Polynomial = self.clone();
        result.integrate(constant);
        result
    }

    /// Adds a polynomial to self
    pub fn add_polynomial(&mut self, other: &Polynomial) {
        let n = self.coefficients.len().max(other.coefficients.len());

        self.coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    + other.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();
    }

    /// Adds a real number x to self
    pub fn add_real(&mut self, x: f64) {
        self.coefficients[0] += x;
    }

    /// Substract a polynomial to self.
    pub fn sub_polynomial(&mut self, other: &Polynomial) {
        let n = self.coefficients.len().max(other.coefficients.len());

        self.coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    - other.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();
    }

    /// Substract a real number x to self.
    pub fn sub_real(&mut self, x: f64) {
        self.coefficients[0] -= x;
    }

    /// Multiply self by a polynomial.
    pub fn mul_by_polynomial(&mut self, other: &Polynomial) {
        let mut result_coefficients =
            vec![0.0; self.coefficients.len() + other.coefficients.len() - 1];

        for (i, self_coeff) in self.coefficients.iter().enumerate() {
            for (j, rhs_coeff) in other.coefficients.iter().enumerate() {
                result_coefficients[i + j] += self_coeff * rhs_coeff;
            }
        }

        self.coefficients = result_coefficients;
    }

    /// Multiply self by a real number x.
    pub fn mul_by_real(&mut self, x: f64) {
        self.coefficients = self.coefficients.iter().map(|coeff| coeff * x).collect();
    }

    /// Divide self by a real number x.
    pub fn div_by_real(&mut self, x: f64) {
        self.coefficients = self.coefficients.iter().map(|coeff| coeff / x).collect();
    }

    /// Returns the real roots of the polynomial
    pub fn real_roots(&self) -> Vec<f64> {
        let degree: usize = self.degree();

        match degree {
            0 => vec![],

            // a x + b = 0
            1 => {
                let b: f64 = self.coefficients[0];
                let a: f64 = self.coefficients[1];

                if a == 0.0 { vec![] } else { vec![-b / a] }
            }

            // a x² + b x + c = 0
            2 => {
                let c: f64 = self.coefficients[0];
                let b: f64 = self.coefficients[1];
                let a: f64 = self.coefficients[2];

                if a == 0.0 {
                    return polynomial![c, b].real_roots();
                }

                let disc: f64 = b * b - 4.0 * a * c;

                if disc > 0.0 {
                    let s: f64 = disc.sqrt();

                    // Numerically stable quadratic formula
                    let q: f64 = -0.5 * (b + b.signum() * s);
                    let r1: f64 = q / a;
                    let r2: f64 = c / q;

                    let mut roots: Vec<f64> = vec![r1, r2];
                    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    roots
                } else if disc.abs() <= 0.0 {
                    vec![-b / (2.0 * a)]
                } else {
                    vec![]
                }
            }

            // a x³ + b x² + c x + d = 0
            3 => {
                let d: f64 = self.coefficients[0];
                let c: f64 = self.coefficients[1];
                let b: f64 = self.coefficients[2];
                let a: f64 = self.coefficients[3];

                if a.abs() < 0.0 {
                    return polynomial![d, c, b].real_roots();
                }

                // Normalize
                let a1: f64 = b / a;
                let a2: f64 = c / a;
                let a3: f64 = d / a;

                // Depressed cubic:
                // t³ + p t + q = 0
                let p: f64 = a2 - a1 * a1 / 3.0;
                let q: f64 = 2.0 * a1.powi(3) / 27.0 - a1 * a2 / 3.0 + a3;

                let disc: f64 = (q / 2.0).powi(2) + (p / 3.0).powi(3);

                if disc > 0.0 {
                    // One real root
                    let sqrt_disc: f64 = disc.sqrt();

                    let u: f64 = (-q / 2.0 + sqrt_disc).cbrt();
                    let v: f64 = (-q / 2.0 - sqrt_disc).cbrt();

                    vec![u + v - a1 / 3.0]
                } else if disc.abs() <= 0.0 {
                    // Multiple roots
                    let u: f64 = (-q / 2.0).cbrt();

                    let r1: f64 = 2.0 * u - a1 / 3.0;
                    let r2: f64 = -u - a1 / 3.0;

                    if (r1 - r2).abs() < 0.0 {
                        vec![r1]
                    } else {
                        let mut roots: Vec<f64> = vec![r1, r2];
                        roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
                        roots
                    }
                } else {
                    // Three distinct real roots
                    let r: f64 = (-p / 3.0).sqrt();
                    let phi: f64 = (-q / (2.0 * r.powi(3))).acos();

                    let x1: f64 = 2.0 * r * (phi / 3.0).cos() - a1 / 3.0;

                    let x2: f64 =
                        2.0 * r * ((phi + 2.0 * std::f64::consts::PI) / 3.0).cos() - a1 / 3.0;

                    let x3: f64 =
                        2.0 * r * ((phi + 4.0 * std::f64::consts::PI) / 3.0).cos() - a1 / 3.0;

                    let mut roots: Vec<f64> = vec![x1, x2, x3];
                    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    roots
                }
            }

            _ => {
                self.companion_matrix().eigenvalues(1.0e-15)
            },
        }
    }
}

// A private helper function to convert a number in superscript
fn superscript(n: usize) -> String {
    let mut number = n;

    const DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

    if number == 0 {
        return "⁰".to_string();
    }

    let mut result = Vec::new();

    while number > 0 {
        result.push(DIGITS[number % 10]);
        number /= 10;
    }

    result.iter().rev().collect()
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for (power, &c) in self.coefficients.iter().enumerate().rev() {
            if c == 0.0 {
                continue;
            }

            if first {
                if c < 0.0 {
                    write!(f, "-")?;
                }
                first = false;
            } else {
                write!(f, " {} ", if c < 0.0 { "-" } else { "+" })?;
            }

            let abs_c = c.abs();

            match power {
                0 => write!(f, "{abs_c}")?,
                1 => {
                    if abs_c != 1.0 {
                        write!(f, "{abs_c}x")?;
                    } else {
                        write!(f, "x")?;
                    }
                }
                _ => {
                    if abs_c != 1.0 {
                        write!(f, "{abs_c}x{}", superscript(power))?;
                    } else {
                        write!(f, "x{}", superscript(power))?;
                    }
                }
            }
        }

        if first {
            write!(f, "0")?;
        }

        Ok(())
    }
}

/// polynomial + polynomial
impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    + rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Self {
            coefficients: result_coefficients,
        }
    }
}

impl Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    + rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

impl Add<Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Polynomial {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    + rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

impl Add<&Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    + rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

/// polynomial + f64
impl Add<f64> for Polynomial {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        let mut result_coefficients = self.coefficients.clone();
        result_coefficients[0] += rhs;
        Self {
            coefficients: result_coefficients,
        }
    }
}

impl Add<f64> for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: f64) -> Polynomial {
        let mut result_coefficients: Vec<f64> = self.coefficients.clone();
        result_coefficients[0] += rhs;
        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

/// f64 + polynomial
impl Add<Polynomial> for f64 {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Polynomial {
        rhs + self
    }
}

impl Add<&Polynomial> for f64 {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        rhs + self
    }
}

/// polynomial - polynomial
impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    - rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Self {
            coefficients: result_coefficients,
        }
    }
}

impl Sub<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Polynomial {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    - rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

impl Sub<Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Polynomial {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    - rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

impl Sub<&Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Polynomial {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    - rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

/// polynomial - f64
impl Sub<f64> for Polynomial {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        let mut result_coefficients = self.coefficients.clone();
        result_coefficients[0] -= rhs;
        Self {
            coefficients: result_coefficients,
        }
    }
}

impl Sub<f64> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: f64) -> Polynomial {
        let mut result_coefficients = self.coefficients.clone();
        result_coefficients[0] -= rhs;
        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

/// f64 - polynomial
impl Sub<Polynomial> for f64 {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Polynomial {
        -rhs + self
    }
}

impl Sub<&Polynomial> for f64 {
    type Output = Polynomial;

    fn sub(self, rhs: &Polynomial) -> Polynomial {
        -rhs + self
    }
}

/// polynomial * polynomial
impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result_coefficients =
            vec![0.0; self.coefficients.len() + rhs.coefficients.len() - 1];

        for (i, self_coeff) in self.coefficients.iter().enumerate() {
            for (j, rhs_coeff) in rhs.coefficients.iter().enumerate() {
                result_coefficients[i + j] += self_coeff * rhs_coeff;
            }
        }

        Self {
            coefficients: result_coefficients,
        }
    }
}

impl Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        let mut result_coefficients =
            vec![0.0; self.coefficients.len() + rhs.coefficients.len() - 1];

        for (i, self_coeff) in self.coefficients.iter().enumerate() {
            for (j, rhs_coeff) in rhs.coefficients.iter().enumerate() {
                result_coefficients[i + j] += self_coeff * rhs_coeff;
            }
        }

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

impl Mul<Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Polynomial {
        let mut result_coefficients =
            vec![0.0; self.coefficients.len() + rhs.coefficients.len() - 1];

        for (i, self_coeff) in self.coefficients.iter().enumerate() {
            for (j, rhs_coeff) in rhs.coefficients.iter().enumerate() {
                result_coefficients[i + j] += self_coeff * rhs_coeff;
            }
        }

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

impl Mul<&Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        let mut result_coefficients =
            vec![0.0; self.coefficients.len() + rhs.coefficients.len() - 1];

        for (i, self_coeff) in self.coefficients.iter().enumerate() {
            for (j, rhs_coeff) in rhs.coefficients.iter().enumerate() {
                result_coefficients[i + j] += self_coeff * rhs_coeff;
            }
        }

        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

/// polynomial * f64
impl Mul<f64> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let result_coefficients = self.coefficients.iter().map(|x| x * rhs).collect();
        Self {
            coefficients: result_coefficients,
        }
    }
}

impl Mul<f64> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: f64) -> Polynomial {
        let result_coefficients: Vec<f64> = self.coefficients.iter().map(|x| x * rhs).collect();
        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

/// f64 * polynomial
impl Mul<Polynomial> for f64 {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Polynomial {
        rhs * self
    }
}

impl Mul<&Polynomial> for f64 {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        rhs * self
    }
}

/// polynomial / f64
impl Div<f64> for Polynomial {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let result_coefficients = self.coefficients.iter().map(|x| x / rhs).collect();
        Self {
            coefficients: result_coefficients,
        }
    }
}

impl Div<f64> for &Polynomial {
    type Output = Polynomial;

    fn div(self, rhs: f64) -> Polynomial {
        let result_coefficients = self.coefficients.iter().map(|x| x / rhs).collect();
        Polynomial {
            coefficients: result_coefficients,
        }
    }
}

/// -polynomial
impl Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self {
        let result_coefficients = self.coefficients.iter().map(|x| -x).collect();
        Self {
            coefficients: result_coefficients,
        }
    }
}

impl Neg for &Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Polynomial {
        let result_coefficients = self.coefficients.iter().map(|x| -x).collect();
        Polynomial {
            coefficients: result_coefficients,
        }
    }
}
