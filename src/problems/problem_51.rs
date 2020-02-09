use crate::helper_modules::prime_generator;
use crate::helper_modules::digits_extractor;
use crate::helper_modules::combination_vector_generator;

/*
    Find the smallest prime which by replacing part of the number with the same digit,
    is part of an eight prime value family
*/
const DEBUG_ENABLED: bool = false;

fn all_digits_same_at_positions(digits: &Vec<u8>, positions: &Vec<u8>) -> bool {
    if positions.len() == 0 {
        return true;
    }
    let value = digits[positions[0] as usize];

    let mut return_value = true;

    for position in positions {
        if digits[*position as usize] != value {
            return_value = false;
        }
    }
    return return_value;
}

fn satisfies_condition(number: u32, sieve: &Vec<usize>, required_count: u8) -> bool {
    let satisfies_condition = false;

    let mut digits: Vec<u8> = digits_extractor::extract_digits(number);

    let all_combinations: Vec<Vec<u8>> =
        combination_vector_generator::generate_combination_vectors_for_size(digits.len());

    if DEBUG_ENABLED {
        println!("Checking condition for prime : {:?} ", number);
    }

    for combination in all_combinations {
        if DEBUG_ENABLED {
            println!("\t Testing combination: {:?}", combination);
        }

        if !all_digits_same_at_positions(&digits, &combination) {
            continue;
        }

        let mut count_of_primes = 0;
        for value in 0..10 {
            if DEBUG_ENABLED {
                println!("\t Replacing with : {:?}", value);
            }
            let new_digits =
                replace_digits_at_positions_with_value(
                    digits.clone(), combination.clone(), value);

            if DEBUG_ENABLED {
                println!("\t new_digits : {:?}", new_digits);
            }

            let new_value = digits_extractor::evaluate_digits(new_digits);
            let new_digits_for_value = digits_extractor::extract_digits(new_value);
            if new_digits_for_value.len() < digits.len() {
                continue;
            }
            if sieve[new_value as usize] == 0 {
                count_of_primes+= 1;
                if DEBUG_ENABLED {
                    println!("\t\tPrime Found! {:?}", new_value);
                }
            }
        }
        if count_of_primes == required_count {
            return true;
        }
    }
    return satisfies_condition;
}

fn replace_digits_at_positions_with_value(digits: Vec<u8>, positions: Vec<u8>, val: u8) -> Vec<u8> {
    let mut new_digits = digits.clone();
    for position in positions {
        new_digits[position as usize] = val;
    }
    return new_digits;
}

pub fn solve(size: usize, required_count: u8) -> usize {
    let sieve_primes_tuple = prime_generator::generate_primes_up_to(size);
    let primes = sieve_primes_tuple.1;
    let mut sieve = sieve_primes_tuple.0;

    sieve[0] = 1;
    sieve[1] = 1;

    let mut answer = 0;

    for prime in primes {
        if satisfies_condition(prime as u32, &sieve, required_count) {
            answer = prime;
            break;
        }
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_for_7() {
        let answer = solve(1000000, 7);
        println!("Answer : {}", answer);
        assert_eq!(answer, 56003);
    }
    #[test]
    fn test_for_120383() {
        let sieve_primes_tuple = prime_generator::generate_primes_up_to(10120384);
        let primes = sieve_primes_tuple.1;
        let mut sieve = sieve_primes_tuple.0;

        sieve[0] = 1;
        sieve[1] = 1;

        println!("{}", sieve[120383]);

        assert_eq!(satisfies_condition(120383, &sieve, 8), false);
    }
}