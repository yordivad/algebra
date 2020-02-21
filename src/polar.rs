use crate::complex::{Complex, Cons};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone, Copy)]
pub struct Polar {
    radius : f64,
    angle: f64
}


impl Polar {
    pub fn new(r: f64, t: f64) -> Polar {
        Polar { radius: r, angle: t }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn angle(&self) -> f64 {
        self.angle
    }
}


impl From<Complex> for Polar {
    fn from(a: Complex) -> Self {
       Polar::new(a.module(), a.angle())
    }
}

impl From<Polar> for Complex {
    fn from(p: Polar) -> Self {
        Complex::new( p.radius * p.angle.cos(), p.radius * p.angle.sin() )
    }
}

impl Display for Polar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:.2}∠{:.3}", self.radius, self.angle)
    }
}
