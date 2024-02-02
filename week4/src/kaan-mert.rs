pub mod week4 {
    pub fn even_list(vector: Vec<i32>) -> Vec<i32> {
        return vector.into_iter().filter(|&temp| temp % 2 == 0).collect();
    }

    pub fn double_list(vector: Vec<i32>) -> Vec<i32> {
        return vector.into_iter().map(|temp| temp * 2).collect();
    }

    pub struct Fib {
       x:(i32,i32)
    }

    impl Fib {
        pub fn new() -> Self {
            Fib { x: (1,2) }
        }
    }

    impl Iterator for Fib {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
                self.x = (self.x.1, self.x.0 * self.x.1);
                Some(self.x.1)
            
        }
    }
}


#[cfg(test)]
mod week4_tests {
    use super::week4::*;

    #[test]
    fn test_even_list() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let v1 = even_list(v1);

        let v2 = vec![1, 3, 5, 7];
        let v2 = even_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![]);
    }

    #[test]
    fn test_double_list() {
        let v1 = vec![1, 2, 3, 4, 5];
        let v1 = double_list(v1);

        let v2 = vec![0, 5, 10, 15];
        let v2 = double_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![0, 10, 20, 30]);
    }

    #[test]
    fn test_multiply_fibonacci() {
        let mut mf_iter = Fib::new();

        assert_eq!(mf_iter.next().unwrap(), 2);
        assert_eq!(mf_iter.next().unwrap(), 4);
        assert_eq!(mf_iter.next().unwrap(), 8);
        assert_eq!(mf_iter.next().unwrap(), 32);
        assert_eq!(mf_iter.next().unwrap(), 256);
        assert_eq!(mf_iter.next().unwrap(), 8192);
    }
}