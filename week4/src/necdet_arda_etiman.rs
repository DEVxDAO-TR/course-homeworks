pub fn even_list(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().filter(|num| num % 2 == 0).collect();
}

pub fn double_list(list: Vec<i32>) -> Vec<i32> {
    return list.into_iter().map(|num| num * 2).collect();
}

pub struct MultiFibonacci {
    pub first: i32,
    pub second: i32
}

impl MultiFibonacci {
    pub fn create() -> Self {
        MultiFibonacci {
            /*first: 0,
            second: 1*/
            first: 1,
            second: 2
        }
    }
}

impl Iterator for MultiFibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.first * self.second;

        self.first = self.second;
        
        self.second = result;
        
        Some(result)
        /*match self.first == 0 && self.second == 1 {
            true => {
                self.first = 1;
                self.second = 1;

                Some(1)
            },
            false => {
                let first_num = self.first;

                self.first = self.second;
                self.second = first_num + self.second;

                Some(self.second)
            }
        }*/
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_list() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let v1 = even_list(v1);

        let v2 = vec![1, 3, 5, 7, 9, 10];
        let v2 = even_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10, 12]);
        assert_eq!(v2, vec![10]);
    }

    #[test]
    fn test_double_list() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7];
        let v1 = double_list(v1);

        let v2 = vec![0, 5, 10, 15, 20, 25];
        let v2 = double_list(v2);

        assert_eq!(v1, vec![2, 4, 6, 8, 10, 12, 14]);
        assert_eq!(v2, vec![0, 10, 20, 30, 40, 50]);
    }

    #[test]
    fn test_multiply_fibonacci() {
        let mut mf_iter = MultiFibonacci::create();

        /*assert_eq!(mf_iter.next().unwrap(), 1);
        assert_eq!(mf_iter.next().unwrap(), 2);
        assert_eq!(mf_iter.next().unwrap(), 3);
        assert_eq!(mf_iter.next().unwrap(), 5);
        assert_eq!(mf_iter.next().unwrap(), 8);
        assert_eq!(mf_iter.next().unwrap(), 13);*/

        assert_eq!(mf_iter.next().unwrap(), 2);
        assert_eq!(mf_iter.next().unwrap(), 4);
        assert_eq!(mf_iter.next().unwrap(), 8);
        assert_eq!(mf_iter.next().unwrap(), 32);
        assert_eq!(mf_iter.next().unwrap(), 256);
        assert_eq!(mf_iter.next().unwrap(), 8192);
    }
}