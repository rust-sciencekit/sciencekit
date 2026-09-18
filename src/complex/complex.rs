use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Default for Complex {
    fn default() -> Self {
        Self::new()
    }
}

impl Complex {
    /// Returns the complex number 0 + 0i
    pub fn new() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    /// Returns the complex number x + yi using the cartesian coordinates (x, y)
    pub fn from_cartesian(x: f64, y: f64) -> Self {
        Self { re: x, im: y }
    }

    /// Returns a new complex number of magnitude r and argument θ using the  polar coordinates (r, θ)
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// The magnitude of a complex number
    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// The squared magnitude of a complex number
    pub fn magnitude_squared(self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }

    /// The argument of a complex number
    pub fn argument(self) -> f64 {
        self.im.atan2(self.re)
    }

    /// The inverse of a complex number z⁻¹
    pub fn inverse(self) -> Self {
        let magnitude_squared: f64 = self.magnitude_squared();
        Self {
            re: self.re / magnitude_squared,
            im: -self.im / magnitude_squared,
        }
    }

    /// The complex conjugate of a complex number
    pub fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// The square root of a complex number
    pub fn sqrt(self) -> Self {
        let x: f64 = (self.magnitude() + self.re).div(2.0).sqrt();
        let y: f64 = (self.magnitude() - self.re).div(2.0).sqrt();
        Self { re: x, im: y }
    }

    /// Returns the complex number raised to an integer exponent
    pub fn powi(self, exponent: i32) -> Self {
        let z: Complex = Complex::from_polar(self.magnitude(), self.argument() * exponent as f64);
        z * self.magnitude().powi(exponent)
    }

    /// Returns the complex number raised to a real exponent
    pub fn powf(self, exponent: f64) -> Self {
        let z: Complex = Complex::from_polar(self.magnitude(), self.argument() * exponent);
        z * self.magnitude().powf(exponent)
    }

    /// Returns the complex number raised to a complex exponent
    pub fn powc(self, exponent: Self) -> Self {
        self.ln().mul(exponent).exp()
    }

    /// Returns the exponential of a complex number
    pub fn exp(self) -> Self {
        Self {
            re: self.re.exp() * self.im.cos(),
            im: self.re.exp() * self.im.sin(),
        }
    }

    /// Returns the natural logarithm of a complex number
    pub fn ln(self) -> Self {
        Self {
            re: f64::ln(self.magnitude()),
            im: self.argument(),
        }
    }

    /// Returns the base-n logarithm of a complex number
    pub fn log(self, base: f64) -> Self {
        self.ln() / base.ln()
    }

    /// The complex sin of a complex number
    pub fn sin(self) -> Self {
        Self {
            re: self.re.sin() * self.im.cosh(),
            im: self.re.cos() * self.im.sinh(),
        }
    }

    /// The complex cosine of a complex number
    pub fn cos(self) -> Self {
        Self {
            re: self.re.cos() * self.im.cosh(),
            im: self.re.sin().neg() * self.im.sinh(),
        }
    }

    /// The complex tangent of a complex number
    pub fn tan(self) -> Self {
        self.sin() / self.cos()
    }

    /// The complex secant of a complex number
    pub fn sec(self) -> Self {
        self.cos().inverse()
    }

    /// The complex cosecant of a complex number
    pub fn csc(self) -> Self {
        self.sin().inverse()
    }

    /// The complex cotangent of a complex number
    pub fn cot(self) -> Self {
        self.tan().inverse()
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im <= 0.0 {
            write!(f, "{} - {}i", self.re, self.im.abs())?;
        } else {
            write!(f, "{} + {}i", self.re, self.im)?;
        }
        Ok(())
    }
}

/// (a+bi) + (c+di)
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

/// (a+bi) + x
impl Add<f64> for Complex {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Self {
            re: self.re + rhs,
            im: self.im,
        }
    }
}

/// x + (a+bi)
impl Add<Complex> for f64 {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: rhs.re + self,
            im: rhs.im,
        }
    }
}

/// (a+bi) - (c+di)
impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

/// (a+bi) - x
impl Sub<f64> for Complex {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Self {
            re: self.re - rhs,
            im: self.im,
        }
    }
}

/// x - (a+bi)
impl Sub<Complex> for f64 {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex {
        -rhs + self
    }
}

/// (a+bi) * (c+di)
impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

/// (a+bi) * x
impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

/// x * (a+bi)
impl Mul<Complex> for f64 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        rhs * self
    }
}

/// (a+bi) / (c+di)
impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / rhs.magnitude_squared(),
            im: (self.im * rhs.re - self.re * rhs.im) / rhs.magnitude_squared(),
        }
    }
}

/// (a+bi) / x
impl Div<f64> for Complex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

/// x / (a+bi)
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Complex> for f64 {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Complex {
        rhs.inverse() * self
    }
}

/// -(a+bi)
impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1_f64
    }
}
