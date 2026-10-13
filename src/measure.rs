use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measure {
    pub value: f64,
    pub error: f64,
}

impl Measure {
    /// The relative error = absolute error / |value|
    pub fn relative_error(self) -> f64 {
        self.error / self.value.abs()
    }
}

/// (x ± a) + (y ± b)
impl Add for Measure {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            value: self.value + rhs.value,
            error: (self.error.powi(2) + rhs.error.powi(2)).sqrt(),
        }
    }
}

/// (x ± a) + y
impl Add<f64> for Measure {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Self {
            value: self.value + rhs,
            error: self.error,
        }
    }
}

/// x + (y ± a)
impl Add<Measure> for f64 {
    type Output = Measure;

    fn add(self, rhs: Measure) -> Measure {
        Measure {
            value: self + rhs.value,
            error: rhs.error,
        }
    }
}

/// (x ± a) - (y ± b)
impl Sub for Measure {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            value: self.value - rhs.value,
            error: (self.error.powi(2) + rhs.error.powi(2)).sqrt(),
        }
    }
}

/// (x ± a) - y
impl Sub<f64> for Measure {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Self {
            value: self.value - rhs,
            error: self.error,
        }
    }
}

/// x - (y ± a)
impl Sub<Measure> for f64 {
    type Output = Measure;

    fn sub(self, rhs: Measure) -> Measure {
        Measure {
            value: self - rhs.value,
            error: rhs.error,
        }
    }
}

/// (x ± a) * (y ± b)
impl Mul for Measure {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let value = self.value * rhs.value;

        Self {
            value,
            error: value.abs()
                * ((self.error / self.value).powi(2) + (rhs.error / rhs.value).powi(2)).sqrt(),
        }
    }
}

/// (x ± a) * y
impl Mul<f64> for Measure {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            value: self.value * rhs,
            error: self.error * rhs.abs(),
        }
    }
}

/// x * (y ± a)
impl Mul<Measure> for f64 {
    type Output = Measure;

    fn mul(self, rhs: Measure) -> Measure {
        rhs * self
    }
}

/// (x ± a) / (y ± b)
impl Div for Measure {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let value = self.value / rhs.value;

        Self {
            value,
            error: ((self.error / rhs.value).powi(2)
                + (self.value * rhs.error / rhs.value.powi(2)).powi(2))
            .sqrt(),
        }
    }
}

/// (x ± a) / y
impl Div<f64> for Measure {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            value: self.value / rhs,
            error: self.error / rhs.abs(),
        }
    }
}

/// x / (y ± a)
impl Div<Measure> for f64 {
    type Output = Measure;

    fn div(self, rhs: Measure) -> Measure {
        Measure {
            value: self / rhs.value,
            error: self.abs() * rhs.error / rhs.value.powi(2),
        }
    }
}

impl Neg for Measure {
    type Output = Measure;

    fn neg(self) -> Measure {
        Measure {
            value: -self.value,
            error: self.error,
        }
    }
}

impl fmt::Display for Measure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.value, self.error)?;
        Ok(())
    }
}
