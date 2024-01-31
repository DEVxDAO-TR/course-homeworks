pub fn hello(name: &str) -> String {
    format!("Hello {}!", name) // Write Your Code Here
}

pub fn make_it_double(num: i32) -> i32 {
    num * 2
}

pub fn multiply_pi(num: f64) -> f64 {
    num * std::f64::consts::PI
}

#[cfg(test)]
mod wecargoek1_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn hello_test() {
        assert_eq!(hello("Emin"), String::from("Hello Emin!"));
        assert_eq!(hello(""), String::from("Hello !"));
        assert_eq!(hello("world"), String::from("Hello world!"));
    }

    #[test]
    fn make_it_double_test() {
        assert_eq!(make_it_double(2), 4);
        assert_eq!(make_it_double(5), 10);
        assert_eq!(make_it_double(-2), -4);
    }

    #[test]
    fn multiply_pi_test() {
        assert_eq!(multiply_pi(1.0), PI);
        assert_eq!(multiply_pi(2.0), PI * 2.0);
        assert_eq!(multiply_pi(5.0), PI * 5.0);
    }
}
