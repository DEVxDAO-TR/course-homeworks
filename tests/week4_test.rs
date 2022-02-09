
#[cfg(test)]
mod week4_tests {
    use course_homeworks::week4;

    #[test]
    fn even_list() {
        let v1 = vec![1,2,3,4,5,6,7,8,9,10];
        let v1 = week4::even_list(v1);

        let v2 = vec![1,3,5,7];
        let v2 = week4::even_list(v2);
        
        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![]);
    }

    #[test]
    fn double_list() {
        let v1 = vec![1,2,3,4,5];
        let v1 = week4::double_list(v1);

        let v2 = vec![0, 5, 10, 15];
        let v2 = week4::double_list(v2);
        
        assert_eq!(v1, vec![2, 4, 6, 8, 10]);
        assert_eq!(v2, vec![0, 10, 20, 30]);
    }

    #[test]
    fn multiply_fibonacci() {
        let mut mf_iter = week4::MultiFibonacci::new();

        assert_eq!(mf_iter.next().unwrap(), 2);
        assert_eq!(mf_iter.next().unwrap(), 4);
        assert_eq!(mf_iter.next().unwrap(), 8);
        assert_eq!(mf_iter.next().unwrap(), 32);
        assert_eq!(mf_iter.next().unwrap(), 256);
        assert_eq!(mf_iter.next().unwrap(), 8192);
    }
}
