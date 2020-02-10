use std::cmp::max;

const DEBUG_ENABLED: bool = false;

pub fn combination(n: u16, r: u16) -> u64 {
    let max_limit = max(r, n-r);
    let mut curr_multiplier = n;
    let mut numerator: u64 = 1;
    while curr_multiplier > max_limit {
        numerator *= curr_multiplier as u64;
        curr_multiplier -= 1;
    }
    let mut denominator: u64 = 1;

    let mut denominator_multiplier =
        max(0, (n as i16 - max_limit as i16 ) as u16);

    while denominator_multiplier >= 1 {
        denominator *= denominator_multiplier as u64;
        denominator_multiplier -= 1;
    }
    if DEBUG_ENABLED {
        println!("{} / {} ", numerator, denominator);
    }
    return numerator/denominator;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_10_c_2() {
        let answer = combination(10, 2);
        let expected = 45;
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_5_c_3() {
        let answer = combination(5, 3);
        let expected = 10;
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_23_c_10() {
        let answer = combination(23, 10);
        let expected = 1144066;
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_23_c0() {
        let answer = combination(23, 0);
        let expected = 1;
        assert_eq!(answer, expected);
    }

    #[test]
    fn test_23_c23() {
        let answer = combination(23, 23);
        let expected = 1;
        assert_eq!(answer, expected);
    }
}