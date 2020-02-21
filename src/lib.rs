pub mod complex;
pub mod polar;
pub mod stats;
pub mod vector;

pub trait Pow {
    fn pow(self, n: f64) -> Self;
}
