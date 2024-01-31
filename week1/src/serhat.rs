
pub fn hello(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn make_it_double(number: i32) -> i32 {
    number * 2
}

pub fn multiply_pi(number: f64) -> f64 {
    number * std::f64::consts::PI
}

#[cfg(test)]
mod week1_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn hello_test() {
        assert_eq!(hello("Serhat"), String::from("Hello Serhat!"));
    }

    #[test]
    fn make_it_double_test() {
        assert_eq!(make_it_double(2),4);
    }

    #[test]
    fn multiply_pi_test() {
        assert_eq!(multiply_pi(6.0), PI * 6.0);

    }
}