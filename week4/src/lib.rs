// Homework - 1
pub fn even_list(v:Vec<i32>) -> Vec<i32> {
    vec![0] // Write Your Code Here
}

// Homework - 2
pub fn double_list(v:Vec<i32>) -> Vec<i32> {
    vec![0] // Write Your Code Here
}

// Homework - 3
pub struct MultiFibonacci {
    first:u32,
    second:u32
}

impl MultiFibonacci {
    pub fn new() -> Self {
        Self {first: 1, second: 2}
    }
} 

impl Iterator for MultiFibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0) // Write Your Code Here
    }

}