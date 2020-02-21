use crate::stats::{Average, Size, Summation};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    data: T,
}

impl<T> Scalar<T> {
    fn scalar(&self) -> &T {
        &self.data
    }
}

impl<T> From<T> for Scalar<T> {
    fn from(value: T) -> Self {
        Scalar { data: value }
    }
}

#[derive(Debug, Clone)]
pub struct Vector<T> {
    components: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: Vec<T>) -> Vector<T> {
        Vector { components: data }
    }
}

impl<T> PartialEq for Vector<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.components
            .iter()
            .zip(other.components.iter())
            .all(|(a, b)| a == b)
    }
}

impl<T> Add for Vector<T>
where
    T: Add<Output = T>,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(
            self.components
                .iter()
                .zip(rhs.components.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<T>>(),
        )
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T>,
    for<'a> &'a T: Sub<&'a T, Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(
            self.components
                .iter()
                .zip(rhs.components.iter())
                .map(|(a, b)| a - b)
                .collect::<Vec<T>>(),
        )
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector::new(self.components.iter().map(|a| a * &rhs).collect())
    }
}

impl<T> Mul<Vector<T>> for Scalar<T>
where
    T: Mul<Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(rhs.components.iter().map(|a| a * &self.scalar()).collect())
    }
}

impl<T> Mul<Vector<T>> for Vector<T>
where
    T: Mul<T, Output = T> + Sum,
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = T;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        self.components
            .iter()
            .zip(rhs.components.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<T> Size for Vector<T> {
    fn n(&self) -> usize {
        self.components.len()
    }
}

impl<T> Summation<T> for Vector<T>
where
    T: Add<Output = T> + Default,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    fn total(&self) -> T {
        self.components.iter().fold(T::default(), |a, i| &a + i)
    }
}

impl<T> Average<T> for Vector<T>
where
    T: Add<Output = T> + Div<usize, Output = T> + Default,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    fn avg(&self) -> T {
        self.total() / self.n()
    }
}
