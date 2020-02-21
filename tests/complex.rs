
    use algebra::complex::*;
    use algebra::Pow;

    #[test]
    fn add_complex() {
        let a = Complex::new(1,2);
        let b = Complex::from((2,1));
        println!("{}", a + b);

        assert_eq!(Complex::new(3,3), a + b);
        assert_eq!(a, a + 0);
        assert_eq!(a, 0 + a);
    }

    #[test]
    fn subs_complex() {
        let a = Complex::new(1, 2);
        let b = Complex::from((2, 1));

        assert_eq!(Complex::from(0), a - a);
        assert_eq!(Complex::from(a), a - 0);
        assert_eq!(Complex::new(-1,1), a - b)
    }

    #[test]
    fn time_complex() {
        let a = Complex::from((2, 3));
        let b = Complex::from((4, 5));
        assert_eq!(Complex::from((-7, 22)), a * b);
        assert_eq!(Complex::from(0), a * 0);
        assert_eq!(Complex::from(0), 0 * a);
        assert_eq!(Complex::new(-1, 0), Complex::i() * Complex::i())
    }

    #[test]
    fn div_complex() {
        let a = Complex::from((3, 2));
        let b = Complex::from((1, -2));

        assert_eq!(Complex::from((-1.0 / 5.0, 8.0 / 5.0)), a / b);
        assert_eq!(-1 * Complex::i(), Complex::from(1) / Complex::i());
        println!("{}", Complex::from(1) / Complex::i())
    }

    #[test]
    fn polar_coordinates() {

    }

    #[test]
    fn pow_complex() {

        let c = Complex::new(4, -4);

      let r =  c.pow(5.0);

        println!("{}", r);
    }
