use std::sync::mpsc::Sender;
use std::thread;


pub fn find_nth_prime_numbers( nums: &[u32], tx: &Sender<u64> ) {
    // tx'i threadlere clone'layıp paralel olarak buldukları asal sayıları tx.send() ile gönder.
    // rx yani Receiver test bölümünde sayıları otomatik toplayacak.

    // Write Your Code Here
    unimplemented!()
}