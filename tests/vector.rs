use algebra::stats::{Average, Size, Summation};
use algebra::vector::{Scalar, Vector};

#[test]
fn vector_addition() {
    let a = Vector::new(vec![1, 2, 3, 4]);
    let b = Vector::new(vec![1, 2, 3, 4]);
    let e = Vector::new(vec![2, 4, 6, 8]);

    assert_eq!(e, a + b);
}

#[test]
fn vector_subtraction() {
    let a = Vector::new(vec![1, 2, 3]);
    let b = Vector::new(vec![1, 2, 3]);
    let e = Vector::new(vec![0, 0, 0]);

    assert_eq!(e, a - b);
}

#[test]
fn scalar_multiplication() {
    let s = 2;
    let a = Vector::new(vec![1, 2, 3, 4]);
    let e = Vector::new(vec![2, 4, 6, 8]);
    assert_eq!(e, a * s);
}

#[test]
fn product_multiplication() {
    let a = Vector::new(vec![3, 4]);
    let b = Vector::new(vec![3, 4]);
    assert_eq!(25, a * b);
}

#[test]
fn average_vector() {
    let a = Vector::new(vec![1, 2, 3, 4, 5]);
    let avg = 3;
    assert_eq!(avg, a.avg());
}

#[test]
fn n_vector() {
    let a = Vector::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(5, a.n())
}

#[test]
fn total_vector() {
    let a = Vector::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(15, a.total())
}
