pub fn even_list(v : Vec<i32>) ->Vec<i32>{

    let mut even_list = Vec::new();

    for i in v{
        if i % 2 == 0{
            even_list.push(i);
        }
    }
    even_list
}


pub fn double_list(v : Vec<i32>) ->Vec<i32>{

    let mut double_list = Vec::new();

    for i in v{
        double_list.push(i*2);
    }
    double_list
}

pub struct MultiFibonacci {
    first: i32,
    second: i32,
}


impl MultiFibonacci {
    pub fn new() -> Self {
        MultiFibonacci {
            first: 1,
            second: 2,
        }
    }
}


impl Iterator for MultiFibonacci {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.first * self.second;
        self.first = self.second;
        self.second = next;
        Some(next)
    }
}


pub fn multiply_fibonacci() -> Vec<i32>{

    let mut fibonacci = Vec::new();
    fibonacci.push(1);
    fibonacci.push(1);

    let mut i = 2;//starts from second element [0] = 1, [1] = 2
    while i < 10{
        fibonacci.push(fibonacci[i-1] + fibonacci[i-2]);
        i += 1;
    }

    let mut multiply_fibonacci = Vec::new();
    for i in fibonacci{
        multiply_fibonacci.push(i*2);
    }
    multiply_fibonacci
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
