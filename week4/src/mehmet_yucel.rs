pub mod week4 {
    pub fn even_list(input: Vec<i32>) -> Vec<i32> {
        input.into_iter().filter(|&x| x % 2 == 0).collect()
    }

    pub fn double_list(input: Vec<i32>) -> Vec<i32> {
        input.into_iter().map(|x| x * 2).collect()
    }

    pub struct MultiFibonacci {
        a: i32,
        b: i32,
    }

    impl MultiFibonacci {
        pub fn new() -> MultiFibonacci {
            MultiFibonacci {
                a: 1,
                b: 2,
            }
        }
    }

    impl Iterator for MultiFibonacci {
        type Item = i32; 

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.a; 
            let new_result = self.a + self.b;
            self.a = self.b;
            self.b = new_result;

            Some(result)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_list() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let v1 = week4::even_list(v1);

        let v2 = vec![1, 3, 5, 7];
        let v2 = week4::even_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![]);
    }

    #[test]
    fn test_double_list() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v1 = week4::double_list(v1);

        let v2 = vec![0, 5, 10, 15];
        let v2 = week4::double_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![0, 10, 20, 30]);
    }

    #[test]
    fn test_multiply_fibonacci() {
        let mut mf_iter = week4::MultiFibonacci::new();

        assert_eq!(mf_iter.next().unwrap(), 2);
        assert_eq!(mf_iter.next().unwrap(), 4);
        assert_eq!(mf_iter.next().unwrap(), 8);
        assert_eq!(mf_iter.next().unwrap(), 32);
        assert_eq!(mf_iter.next().unwrap(), 256);
        assert_eq!(mf_iter.next().unwrap(), 8192);
    }
}
