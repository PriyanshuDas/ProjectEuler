
pub fn generate_primes_up_to(n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut primes: Vec<usize> = vec![];
    let mut sieve = vec![0; n+1];
    for number in 2..n+1 {
        if sieve[number] == 1 {
            continue;
        }
        let mut cur_multiple = number*number;
        while cur_multiple <= n {
            sieve[cur_multiple] = 1;
            cur_multiple += number;
        }
        primes.push(number);
    }
    return(sieve, primes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_generation_10() {
        let generated_primes = generate_primes_up_to(10).1;
        let expected_primes: Vec<usize> = vec![2, 3, 5, 7];
        assert_eq!(generated_primes, expected_primes);
    }
    #[test]
    fn test_prime_generation_100() {
        let generated_primes = generate_primes_up_to(100).1;
        let expected_primes: Vec<usize> =
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
                 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

        assert_eq!(generated_primes, expected_primes);
    }
}