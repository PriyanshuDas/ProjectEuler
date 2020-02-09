use combinations::Combinations;

pub fn generate_combination_vectors_for_size(n: usize) -> Vec<Vec<u8>> {
    let mut combinations = vec![];
//    combinations.push(vec![vec![]]);
    let mut list: Vec<u8> = generate_sorted_permutation_of_size(n);
    for size in 1..n {
        let combinations1 = Combinations::new(list.clone(), size);
        let combination: Vec<_> = combinations1.collect();
        combinations.push(combination);
    }
//    combinations.push(vec![list]);

    return combinations.into_iter().flatten().collect();
}

fn generate_sorted_permutation_of_size(n: usize) -> Vec<u8> {
    let mut list = vec![];
    for item in 0..n {
        list.push(item as u8);
    }
    return list;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let actual_values = generate_combination_vectors_for_size(3);
        let expected_values = vec![
//            vec![],
            vec![0],
            vec![1],
            vec![2],
            vec![0, 1],
            vec![0, 2],
            vec![1, 2],
//            vec![0, 1, 2]
        ];
        assert_eq!(actual_values, expected_values);
    }
}