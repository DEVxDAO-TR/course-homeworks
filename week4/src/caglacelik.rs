pub fn even_list(list: Vec<u32>) -> Vec<u32> {
    list.into_iter().filter(|el| el % 2 == 0).collect()
}

pub fn double_list(list: Vec<u32>) -> Vec<u32> {
    list.into_iter().map(|el| el * 2).collect()
}

struct MultiFibonacci {
    previous: u32,
    current: u32,
}

impl Default for MultiFibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFibonacci {
    pub fn new() -> Self {
        Self {
            previous: 1,
            current: 2,
        }
    }
}

impl Iterator for MultiFibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.previous * self.current;
        self.previous = self.current;
        self.current = res;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut mf_iter = MultiFibonacci::new();

        assert_eq!(mf_iter.next().unwrap(), 2);
        assert_eq!(mf_iter.next().unwrap(), 4);
        assert_eq!(mf_iter.next().unwrap(), 8);
        assert_eq!(mf_iter.next().unwrap(), 32);
        assert_eq!(mf_iter.next().unwrap(), 256);
        assert_eq!(mf_iter.next().unwrap(), 8192);
    }
}
