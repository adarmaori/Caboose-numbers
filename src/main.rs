use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_prime(n: u64, primes: Vec<u64>) -> bool {
    if n > primes[primes.len() - 1] {
        for i in &primes {
            if n % i == 0 {
                return false;
            }
            if i * i > n {
                return true;
            }
        }
    }

    let mut left = 0;
    let mut right = primes.len() - 1;
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
    for i in 3..n {
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
    let mut candidates: Vec<(u64, usize)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let prime: u64 = line.trim().parse().unwrap();
        primes.push(prime);
    }
    for i in 0..primes.len() - 1 {
        if primes[i + 1] == primes[i] + 2 {
            candidates.push((primes[i].clone(), i));
        }
    }
    let mut index = 2;
    loop {
        let (num, p_index) = candidates[index];
        if is_caboose(num, primes[p_index+1..].to_vec()) {
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
