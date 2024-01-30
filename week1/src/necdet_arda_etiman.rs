pub fn hello(name: &str) -> String {
    format!("hello {}!", name)
}

pub fn make_it_double(num: i32) -> i32 {
    return match num % 2 {
        0 => num * 2,
        _ => num
    }
}

pub fn multiply_pi(num: f64) -> f64 {
    use std::f64::consts::PI;

    return num * PI // Write Your Code Here
}

#[cfg(test)]
mod week1_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn hello_test() {
        assert_eq!(hello("Necdet"), String::from("hello Necdet!"));
        assert_eq!(hello(""), String::from("hello !"));
        assert_eq!(hello("world"), String::from("hello world!"));
    }

    #[test]
    fn make_it_double_test() {
        assert_eq!(make_it_double(2), 4);
        assert_eq!(make_it_double(5), 5);
        assert_eq!(make_it_double(-2), -4);
    }

    #[test]
    fn multiply_pi_test() {
        assert_eq!(multiply_pi(1.0), PI);
        assert_eq!(multiply_pi(2.0), PI * 2.0);
        assert_eq!(multiply_pi(5.0), PI * 5.0);
    }
}