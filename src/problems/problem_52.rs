use super::super::helper_modules::digits_extractor;

const DEBUG_ENABLED: bool = false;

fn validate_all_digits_sets_equal(digits_list: &mut Vec<Vec<u8>>) -> bool {
    let mut return_value = true;
    for digits in digits_list.iter_mut() {
        digits.sort();
    }
    if DEBUG_ENABLED {
        println!("\t{:?}", digits_list);
    }
    let first_set = digits_list[0].clone();
    for digits in digits_list {
        if *digits != first_set {
            return_value = false;
        }
    }
    return  return_value;
}

pub fn solve(lower_range: usize, upper_range: usize, count: usize) -> usize {
    let mut answer = 0;
    for value in lower_range..upper_range+1 {
        let mut digits_list = vec![];
        for multiple in 1..count+1 {
            digits_list.push(
                digits_extractor::extract_digits( (value * multiple) as u32));
        }
        if validate_all_digits_sets_equal(&mut digits_list) {
            answer = value;
            break;
        }
    }
    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_for_2() {
        let solution = solve(125874, 125875, 2);
        assert_eq!(solution, 125874);
    }
}