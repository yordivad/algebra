pub trait Size {
    fn n(&self) -> usize;
}

pub trait Summation<T> {
    fn total(&self) -> T;
}

pub trait Average<T> {
    fn avg(&self) -> T;
}
