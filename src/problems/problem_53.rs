/*
How many, not necessarily distinct, values of (n C r) for 1≤n≤100,
 are greater than one-million?
*/

use super::super::helper_modules::permutations_and_combinations;

const DEBUG_ENABLED: bool = false;

pub fn solve(left: u16, right: u16, limit: u64) -> u16 {
    let mut total_count: u16 = 0;
    for number in left..right+1 {
        let mut left_limit: u16 = number;
        for r in 0..(number+1)/2 {
            let curr_combination = permutations_and_combinations::combination(number, r);
            if curr_combination > limit {
                if DEBUG_ENABLED {
                    println!(" {}, {} == {}", number, r, curr_combination);
                }
                left_limit = r - 1;
                break;
            }
        }
        if left_limit == number {
            if number % 2 == 0 {
                let middle_combination =
                    permutations_and_combinations::combination(number, number/2);
                if middle_combination > limit {
                    total_count += 1;
                }
            }
            continue;
        }

        total_count += number + 1 - 2*(left_limit + 1);
    }
    return total_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23_10() {
        assert_eq!(solve(23, 23, 1000000), 4);
    }
}