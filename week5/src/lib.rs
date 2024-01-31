automod::dir!(pub "src/");

// Test Templates
#[cfg(test)]
#[cfg(target_os = "windows")] // burayı kendi kodunuzda silin
mod week5_tests {
    use std::sync::mpsc::{channel, Receiver, Sender};
    use week5;

    #[test]
    fn find_nth_prime_numbers() {
        let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();
        let numbers1: [u32; 4] = [1, 2, 3, 4];
        let numbers2: [u32; 4] = [1000, 2000, 3000, 4000];
        let numbers3: [u32; 4] = [5000, 10000, 15000, 20000];

        week5::find_nth_prime_numbers(&numbers1, &tx);

        let mut result1: u64 = 0;
        for _i in 0..4 {
            result1 += rx.recv().unwrap();
        }

        week5::find_nth_prime_numbers(&numbers2, &tx);

        let mut result2: u64 = 0;
        for _i in 0..4 {
            result2 += rx.recv().unwrap();
        }

        week5::find_nth_prime_numbers(&numbers3, &tx);

        let mut result3: u64 = 0;
        for _i in 0..4 {
            result3 += rx.recv().unwrap();
        }

        assert_eq!(result1, 17);
        assert_eq!(result2, 90570);
        assert_eq!(result3, 541918);
    }
}
