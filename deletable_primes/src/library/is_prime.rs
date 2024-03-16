pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn deletable_primes(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    deletable_primes_recursive(n, &mut result);
    result
}

fn deletable_primes_recursive(n: u64, result: &mut Vec<u64>) {
    let n_str = n.to_string();
    for i in 0..n_str.len() {
        let mut new_n_str = n_str.clone();
        new_n_str.remove(i);
        let new_n = match new_n_str.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if is_prime(new_n) {
            result.push(new_n);
            deletable_primes_recursive(new_n, result);
        }
    }
}

pub fn count_single_digit_numbers(numbers: Vec<u64>) -> u64 {
    let mut count = 0;
    for n in numbers {
        if n < 10 {
            count += 1;
        }
    }
    count
}
