use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_prime(n: u64, primes: Vec<u64>) -> bool {
    if n > primes[primes.len() - 1] {
        for i in primes {
            if n % i == 0 {
                return false;
            }
            if i * i > n {
                return true;
            }
        }
    }
    let min = 0;
    let max = (n as usize) / 2 + 1;
    let mut left = min;
    let mut right = max;
    while left <= right {
        let mid = (left + right) / 2;
        if primes[mid] == n {
            return true;
        } else if primes[mid] < n {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}

fn is_caboose(n: u64, primes: Vec<u64>) -> bool {
    for i in 2..n {
        if !is_prime(i * (i - 1) + n, primes.clone()) {
            return false;
        }
    }
    true
}


fn main() {
    // Read the primes from the "primes.txt" file
    let file = File::open("primes.txt").unwrap();
    let reader = BufReader::new(file);
    let mut primes: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let prime: u64 = line.trim().parse().unwrap();
        primes.push(prime);
    }
    let mut index = 2;
    loop {
        let num = primes[index];
        if is_caboose(num, primes.clone()) {
            println!("{} is a caboose number", num);
            if num > 41 {
                break;
            }
        }
        if index % 100 == 0 {
            println!("checked up to {}", num);
        }
        index += 1;
    }
}
