pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let name = "Mehmet";
    let greeting = hello(name);
    println!("{}", greeting);
}
//--------------------------------------------------------
pub fn make_it_double(num: i32) -> i32 {
    num * 2
}

fn main() {
    let number = 5;
    let doubled_number = make_it_double(number);
    println!("Double of {}: {}", number, doubled_number);
}
//-----------------------------------------------

use std::f64::consts::PI;

pub fn multiply_pi(num: f64) -> f64 {
    num * PI
}

fn main() {
    let number = 3.0;
    let result = multiply_pi(number);
    println!("{} multiplied by π is: {}", number, result);
}

//-----------------------------------------------------------------

#[cfg(test)]
mod week1_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn hello_test() {
        assert_eq!(hello("Mehmet"), String::from("Selam Mehmet!"));
        assert_eq!(hello(""), String::from("hii !"));
        assert_eq!(hello("world"), String::from("hiii world!"));
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