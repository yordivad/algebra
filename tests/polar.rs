use algebra::complex::{Complex, Cons};
use algebra::polar::Polar;

#[test]
fn complex_to_polar() {
    let c = Complex::new(3, 4);

    let p: Polar = c.into();
    let cp: Complex = p.into();
    println!("{}", p);
    println!("{}", cp);
}