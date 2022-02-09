#[cfg(test)]
mod week1_tests {
    use std::f64::consts::PI;
    use week1;
    
    #[test]
    fn hello() {
        assert_eq!(week1::hello("Emin"), String::from("Hello Emin!"));
        assert_eq!(week1::hello(""), String::from("Hello !"));
        assert_eq!(week1::hello("world"), String::from("Hello world!"));
    }

    #[test]
    fn make_double() {
        assert_eq!(week1::make_it_double(2), 4);
        assert_eq!(week1::make_it_double(5), 10);
        assert_eq!(week1::make_it_double(-2), -4);
    }

    #[test]
    fn multiply_pi() {
        assert_eq!(week1::multiply_pi(1.0), PI);
        assert_eq!(week1::multiply_pi(2.0), PI*2.0);
        assert_eq!(week1::multiply_pi(5.0), PI*5.0);
    }
}