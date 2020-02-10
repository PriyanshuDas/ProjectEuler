const SUITS: [&str; 4] = ["C", "S", "D", "H"];

const RANKS: [&str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

fn hand_contains_cards_to_find(hand: &Vec<&str>, cards_to_find: Vec<String>) -> bool {
    let mut contains_all = true;
    for card_to_find in cards_to_find {
        let mut card_found = false;
        for card in hand {
            if card.to_string() == card_to_find {
                card_found = true;
            }
        }
        if !card_found {
            contains_all = false;
            break;
        }
    }
    return contains_all;
}

pub fn hand_has_royal_flush(hand: &Vec<&str>) -> bool {
    let mut return_val: bool = false;
    let ranks_to_find = vec!["A", "K", "Q", "J", "10"];
    for suit in SUITS.iter() {
        let mut cards_to_find = vec![];
        for rank in ranks_to_find.iter() {
            cards_to_find.push((*rank).to_string() + &(*suit).to_string());
        }
        if hand_contains_cards_to_find(hand, cards_to_find) {
            return_val = true;
            break;
        }
    }
    return return_val;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_has_royal_flush() {
        let hand: Vec<&str> = vec!["AS", "KS", "QS", "JS", "10S"];
        let expected_output = true;
        let actual_output = hand_has_royal_flush(&hand);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_hand_doesnt_have_royal_flush() {
        let hand: Vec<&str> = vec!["AD", "KS", "QS", "JS", "10S"];
        let expected_output = false;
        let actual_output = hand_has_royal_flush(&hand);
        assert_eq!(actual_output, expected_output);
    }
}