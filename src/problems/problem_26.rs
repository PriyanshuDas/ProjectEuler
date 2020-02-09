use rayon::prelude::*;
use std::ops::Range;

const DEBUG_ENABLED: bool = false;

pub fn get_remainder(dividend: i64, divisor: i64) -> i64 {
    let remainder = dividend % divisor;
    if DEBUG_ENABLED {
        println!("\t\t Remainder for : {} % {} = {}", dividend, divisor, remainder);
    }
    return remainder;
}

pub fn check_list_contains_item<T: std::cmp::PartialEq>(list: &Vec<T>, item: T) -> bool {
    for value in list {
        if *value == item {
            return true;
        }
    }
    return false;
}

pub fn get_remainder_list(divisor: usize) -> (Vec<u64>, bool) {
    let mut remainder_list: Vec<u64> = vec![];
    let mut cur_dividend: i64 = 1;
    let mut remainder = get_remainder(cur_dividend, divisor as i64);

    if DEBUG_ENABLED {
        println!("\tStarting Remainder: {}", remainder);
    }
    while !(remainder == 0 ||
        check_list_contains_item(&remainder_list, remainder as u64)) {
        remainder_list.push(remainder as u64);
        if remainder < divisor as i64 {
            remainder *= 10;
        }

        remainder = get_remainder(remainder, divisor as i64);
    }

    if remainder == 0 {
        return (vec![], false);
    }

    remainder_list.push(remainder as u64);
    return (remainder_list, true);
}

pub fn get_longest_recurring_cycle_for_divisor(divisor: usize) -> usize {
    if DEBUG_ENABLED {
        println!("Calling get_longest_recurring_cycle_for_divisor: {}", divisor);
    }
    let remainder_list_response = get_remainder_list(divisor);

    if !remainder_list_response.1 {
        return 0;
    }

    let remainder_list = remainder_list_response.0;
    if DEBUG_ENABLED {
        println!("\t Remainder_List: {:?}", remainder_list);
    }

    if remainder_list.len() < 1 {
        return 0;
    }
    let last_remainder = remainder_list.get(remainder_list.len() - 1);
    let last_position = remainder_list.len() - 1;

    let mut first_position = 0;

    for i in 0..remainder_list.len() {
        if remainder_list.get(i) == last_remainder {
            first_position = i;
            break;
        }
    }
    return last_position - first_position;
}

pub fn solve(lower_limit: usize, upper_limit: usize) -> (usize, usize) {
    let mut answer = 0;
    let mut answer_size = 0;
    let mut range = (lower_limit..(upper_limit + 1)).collect::<Vec<usize>>();
    let answers_pairs: Vec<(usize, usize)> =
        range.par_iter_mut()
            .map(|value| {
                let cur_size = get_longest_recurring_cycle_for_divisor(*value);
                return (cur_size, *value);
            }).collect();

    return *answers_pairs.iter().max().unwrap();
}