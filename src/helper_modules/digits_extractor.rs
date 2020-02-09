use rand::Rng;

pub fn extract_digits(n: u32) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];
    let mut value = n;
    while value > 0 {
        digits.push((value%10) as u8);
        value = value/10;
    }
    return digits;
}

pub fn evaluate_digits(digits: Vec<u8>) -> u32 {
    let mut value: u32 = 0;
    let mut digit: u64 = 1;
    for position in 0..digits.len() {
        value += (digits[position] as u32)*(digit as u32);
        digit *= 10;
    }
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digits_12345() {
        let extracted_digits = extract_digits(12345);
        let expected_digits = vec![5, 4, 3, 2, 1];
        assert_eq!(expected_digits, extracted_digits);
    }

    #[test]
    fn test_evaluate_digits_12345() {
        let extracted_digits = evaluate_digits( vec![5, 4, 3, 2, 1]);
        let expected_digits = 12345;
        assert_eq!(extracted_digits, expected_digits);
    }

    #[test]
    fn test_extract_digits_54321() {
        let extracted_digits = extract_digits(54321);
        let expected_digits = vec![1, 2, 3, 4, 5];
        assert_eq!(expected_digits, extracted_digits);
    }

    #[test]
    fn test_evaluate_digits_54321() {
        let extracted_digits = evaluate_digits( vec![1, 2, 3, 4, 5]);
        let expected_digits = 54321;
        assert_eq!(extracted_digits, expected_digits);
    }

    #[test]
    fn test_extract_digits_1000() {
        let extracted_digits = extract_digits(1000);
        let expected_digits = vec![0, 0, 0, 1];
        assert_eq!(expected_digits, extracted_digits);
    }

    #[test]
    fn test_evaluate_digits_1000() {
        let extracted_digits = evaluate_digits(vec![0, 0, 0, 1]);
        let expected_digits = 1000;
        assert_eq!(extracted_digits, expected_digits);
    }

    #[test]
    fn test_random_generation_tests() {
        let mut rng = rand::thread_rng();
        for test_case in 0..10000 {
            let random_number: u32 = rng.gen();
            let extracted_digits = extract_digits(random_number);
            let evaluated_digits = evaluate_digits(extracted_digits);
            assert_eq!(evaluated_digits, random_number);
        }
    }
}