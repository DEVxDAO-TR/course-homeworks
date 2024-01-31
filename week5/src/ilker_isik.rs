use std::sync::mpsc::Sender;
use std::thread;

/// Returns the nth prime numer.
/// 1 -> 2
/// 2 -> 3
/// 3 -> 5
pub fn nth_prime(n: u32) -> u64 {
    let mut count = 0;
    for num in 2u64.. {
        // Largest possible factor is square root of num
        let upper = ((num as f64).sqrt().ceil() as u64).min(num - 1);
        let is_prime = (2..=upper).all(|i| num % i != 0);
        if is_prime {
            count += 1;
            if count == n {
                return num;
            }
        }
    }
    unreachable!()
}

/// Returns the nth prime numer.
/// Sieve algorithm.
/// 1 -> 2
/// 2 -> 3
/// 3 -> 5
pub fn sieve_nth_prime(n: u32) -> u64 {
    const MAX_NUM: usize = 541918;
    let mut is_prime: [bool; MAX_NUM + 1] = [true; MAX_NUM + 1];
    let mut num = 0;
    for i in 2..=MAX_NUM {
        if is_prime[i] {
            for j in (i*i..=MAX_NUM).step_by(i) {
                is_prime[j] = false;
            }
            num += 1;
            if n == num {
                return i as u64;
            }
        }
    }
    unreachable!()
}

pub fn find_nth_prime_numbers(nums: &[u32], tx: &Sender<u64>) {
    // Create a thread for each element in nums
    for num in nums {
        let tx = tx.clone();
        let num = *num;
        thread::spawn(move || {
            // Get nth prime
            let prime = nth_prime(num);
            // Send prime over channel
            tx.send(prime).unwrap();
        });
    }
}

// Test Templates
#[cfg(test)]
mod week5_tests {
    use std::sync::mpsc::{channel, Receiver, Sender};

    #[test]
    fn test_sieve_nth_prime() {
        assert_eq!(super::sieve_nth_prime(1), 2);
        assert_eq!(super::sieve_nth_prime(2), 3);
        assert_eq!(super::sieve_nth_prime(3), 5);
        assert_eq!(super::sieve_nth_prime(4), 7);
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(super::nth_prime(1), 2);
        assert_eq!(super::nth_prime(2), 3);
        assert_eq!(super::nth_prime(3), 5);
        assert_eq!(super::nth_prime(4), 7);
    }

    #[test]
    fn find_nth_prime_numbers() {
        let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();
        let numbers1: [u32; 4] = [1, 2, 3, 4];
        let numbers2: [u32; 4] = [1000, 2000, 3000, 4000];
        let numbers3: [u32; 4] = [5000, 10000, 15000, 20000];

        super::find_nth_prime_numbers(&numbers1, &tx);

        let mut result1: u64 = 0;
        for _i in 0..4 {
            result1 += rx.recv().unwrap();
        }

        super::find_nth_prime_numbers(&numbers2, &tx);

        let mut result2: u64 = 0;
        for _i in 0..4 {
            result2 += rx.recv().unwrap();
        }

        super::find_nth_prime_numbers(&numbers3, &tx);

        let mut result3: u64 = 0;
        for _i in 0..4 {
            result3 += rx.recv().unwrap();
        }

        assert_eq!(result1, 17);
        assert_eq!(result2, 90570);
        assert_eq!(result3, 541918);
    }
}
