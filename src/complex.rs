use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Formatter, Error};
use crate::Pow;
use crate::polar::Polar;


#[derive(Debug, Clone, Copy)]
pub struct Complex {
    real: f64,
    img: f64,
}


pub trait Cons<T> {
    fn new(real: T, img: T) -> Self;
}

impl Complex {
    pub fn i() -> Complex {
        Complex::new(0, 1)
    }

    pub fn adjoin(&self) -> Complex {
        Complex::new(self.real, self.img * -1.0)
    }

    pub fn angle(&self) -> f64 {
        self.img.atan2(self.real)
    }

    pub fn module(&self) -> f64 {
        (self.real * self.real + self.img * self.img).sqrt()
    }
}

impl Cons<i32> for Complex {
    fn new(real: i32, img: i32) -> Self {
        Complex::from((real as f64, img as f64))
    }
}

impl Cons<f64> for Complex {
    fn new(real: f64, img: f64) -> Self {
        Complex::from((real, img))
    }
}

impl Cons<usize> for Complex {
    fn new(real: usize, img: usize) -> Self {
        Complex::from((real as f64, img as f64))
    }
}

impl Cons<isize> for Complex {
    fn new(real: isize, img: isize) -> Self {
        Complex::from((real as f64, img as f64))
    }
}

impl From<f64> for Complex {
    fn from(real: f64) -> Self {
        Complex { real, img: 0.0 }
    }
}

impl From<isize> for Complex {
    fn from(r: isize) -> Self {
        Complex::from(r as f64)
    }
}

impl From<(f64, f64)> for Complex {
    fn from((r, i): (f64, f64)) -> Self {
        Complex { real: r, img: i }
    }
}

impl From<(isize, isize)> for Complex {
    fn from((r, i): (isize, isize)) -> Self {
        Complex::from((r as f64, i as f64))
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex { real: self.real + rhs.real, img: self.img + rhs.img }
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.img == other.img
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, " {:.2}{}{:.2}i", self.real, if self.img > 0.0 { "+" } else { "" }, self.img)
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex::new(self.real - rhs.real, self.img - rhs.img)
    }
}

impl Add<f64> for Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        self + Complex::from(rhs)
    }
}

impl Add<i32> for Complex {
    type Output = Complex;

    fn add(self, rhs: i32) -> Self::Output {
        self + Complex::from(rhs as f64)
    }
}


impl Add<Complex> for f64 {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex::from(self) + rhs
    }
}


impl Add<Complex> for i32 {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex::from(self as f64) + rhs
    }
}


impl Sub<f64> for Complex {
    type Output = Complex;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Complex::from(rhs)
    }
}

impl Sub<i32> for Complex {
    type Output = Complex;

    fn sub(self, rhs: i32) -> Self::Output {
        self - Complex::from(rhs as f64)
    }
}

impl Sub<Complex> for f64 {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex::from(self) - rhs
    }
}


impl Sub<Complex> for i32 {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex::from(self as f64) - rhs
    }
}


impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Complex::from(rhs) * self
    }
}

impl Mul<i32> for Complex {
    type Output = Complex;

    fn mul(self, rhs: i32) -> Self::Output {
        self * rhs as f64
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        let r = self.real * rhs.real;
        let i = self.img * rhs.img;
        let ri = self.real * rhs.img;
        let ir = self.img * rhs.real;

        Complex::new(r - i, ri + ir)
    }
}

impl Mul<Complex> for f64 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex::from(self) * rhs
    }
}

impl Mul<Complex> for i32 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex::from(self as f64) * rhs
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        let n = self * rhs.adjoin();
        let d = rhs * rhs.adjoin();
        Complex::new(n.real / d.real, n.img / d.real)
    }
}

impl Pow for Complex {
    fn pow(self, n: f64) -> Self {
        let p = Polar::from(self);
        let pow = p.radius().powf(n);
        Complex::new(pow * (n * p.angle()).cos(), pow * (n * p.angle()).sin())
    }
}


