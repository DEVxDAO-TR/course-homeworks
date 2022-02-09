use std::f64::consts::PI;

pub fn hello(name:&str) -> String {
    format!("Hello {}!", name)
}

pub fn make_it_double(num:i32) -> i32 {
    num * 2
}

pub fn multiply_pi(num:f64) -> f64 {
    num * PI
}