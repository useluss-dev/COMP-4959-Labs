use std::collections::HashMap;

fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;

    while p <= n / p {
        if is_prime[p] {
            let mut multiple = p * p;

            while multiple <= n {
                is_prime[multiple] = false;
                multiple += p;
            }
        }

        p += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(number, &prime)| prime.then_some(number))
        .collect()
}

fn primes(m: usize, n: usize) -> Vec<usize> {
    if m > n || n < 2 {
        return Vec::new();
    }

    sieve(n).into_iter().filter(|&p| p >= m).collect()
}

fn main() {
    let sorted_primes: Vec<_> = primes(100_000, 1_000_000)
        .iter()
        .map(|x| {
            let mut digits: Vec<char> = x.to_string().chars().collect();
            digits.sort_unstable();
            digits.into_iter().collect::<String>()
        })
        .collect();
    println!("{}, 6 digit primes", sorted_primes.len());

    let mut counts = HashMap::new();
    for value in sorted_primes {
        *counts.entry(value).or_insert(0) += 1;
    }
    if let Some((_, count)) = counts.into_iter().max_by_key(|(_, count)| *count) {
        println!(
            "Largest set of 6 digit primes that are permutations of each other is {count} primes."
        );
    }
}
