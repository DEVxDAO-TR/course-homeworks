// Homework - 1
pub fn even_list(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().filter(|&x| x % 2 == 0).collect()
}

// Homework - 2
pub fn double_list(v: Vec<i32>) -> Vec<i32> {
    v.into_iter().map(|x| x * 2).collect()
}

// Homework - 3
pub struct MultiFibonacci {
    first: u32,
    second: u32,
}

impl Default for MultiFibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFibonacci {
    pub fn new() -> Self {
        Self {first: 1, second: 2}
    }
} 

impl Iterator for MultiFibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.first * self.second;
        self.first = self.second;
        self.second = result;
        Some(result)
    }

}

#[cfg(test)]
mod week4_tests {
    #[test]
    fn even_list() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let v1 = super::even_list(v1);

        let v2 = vec![1, 3, 5, 7];
        let v2 = super::even_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![]);
    }

    #[test]
    fn double_list() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v1 = super::double_list(v1);

        let v2 = vec![0, 5, 10, 15];
        let v2 = super::double_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![0, 10, 20, 30]);
    }

    #[test]
    fn multiply_fibonacci() {
        let mut mf_iter = super::MultiFibonacci::new();

        assert_eq!(mf_iter.next().unwrap(), 2);
        assert_eq!(mf_iter.next().unwrap(), 4);
        assert_eq!(mf_iter.next().unwrap(), 8);
        assert_eq!(mf_iter.next().unwrap(), 32);
        assert_eq!(mf_iter.next().unwrap(), 256);
        assert_eq!(mf_iter.next().unwrap(), 8192);
    }
}
