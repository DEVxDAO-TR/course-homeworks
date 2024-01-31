pub fn even_list(v: Vec<i32>) -> Vec<i32>{
    v.into_iter().filter(|x| x % 2 == 0).collect()
}

pub fn double_list(v: Vec<i32>) -> Vec<i32>{
    v.into_iter().map(|x| x * 2).collect()
}

pub struct MultiFibonacci {
    a: u32,
    b: u32,
}

impl Default for MultiFibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFibonacci {
    pub fn new() -> Self {
        Self {a: 1, b: 2}
    }
} 

impl Iterator for MultiFibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a * self.b;
        self.a = self.b;
        self.b = c;

        Some(c)
    }

}
#[cfg(test)]
mod week4_tests {
    use super::*;

    #[test]
    fn even_list_test() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let v1 = even_list(v1);

        let v2 = vec![1, 3, 5, 7];
        let v2 = even_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![]);
    }

    #[test]
    fn double_list_test() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v1 = double_list(v1);

        let v2 = vec![0, 5, 10, 15];
        let v2 = double_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![0, 10, 20, 30]);
    }

    #[test]
    fn multiply_fibonacci_test() {
        let mut mf_iter = MultiFibonacci::new();

        assert_eq!(mf_iter.next().unwrap(), 2);
        assert_eq!(mf_iter.next().unwrap(), 4);
        assert_eq!(mf_iter.next().unwrap(), 8);
        assert_eq!(mf_iter.next().unwrap(), 32);
        assert_eq!(mf_iter.next().unwrap(), 256);
        assert_eq!(mf_iter.next().unwrap(), 8192);
    }
}
