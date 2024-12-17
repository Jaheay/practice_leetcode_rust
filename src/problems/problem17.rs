use std::collections::HashMap;

pub struct Solution;

impl Solution {


    pub fn letter_combinations(digits: String) -> Vec<String> {
        
        // If input empty, output empty
        if digits.is_empty() {
            return vec![];
        }

        if digits.len() > 4 {
            panic!("Digits must not be greater than length 4")
        }

        // Hashmap of phone keys
        let phone_number_to_letters: HashMap<char, Vec<char>> = [
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]
        .iter()
        .cloned()
        .collect();

        // Process each digit to generate all possible letter combinations.
        // - `fold`: Iteratively builds up combinations, starting with an empty string.
        // - For each `digit`, retrieve its corresponding `letters` from the `phone_number_to_letters` map.
        // - Use `flat_map` to append each `letter` to every existing combination in `current_combinations`:
        //      - `current_combinations.iter().map(...)`: Creates new combinations by appending the letter.
        //      - `flat_map`: Flattens all new combinations into a single vector.
        // - `collect`: Converts the iterator back into a Vec<String> for the next iteration.
        // If an invalid digit is encountered, panic immediately.

        digits.chars().fold(vec![String::new()], | current_combinations, digit | {
            if let Some(letters) = phone_number_to_letters.get(&digit) {
                current_combinations
                .iter() // Iterate over `current_combinations` first
                .flat_map(|combination| {
                    letters.iter().map(move |&letter| format!("{}{}", combination, letter))
                })
            .collect()
            } else {
                panic!("Invalid")
            }
        })


    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Input: digits = "23"
    /// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
    fn example1() { 
        let input = "23".to_string();
        let output = Solution::letter_combinations(input);

        // Hawt Garbo, but it works
        let example_output = Vec::from(["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].map(String::from));
        assert_eq!(output, example_output)
    }

    #[test]
    /// Input: digits = ""
    /// Output: []
    fn example2() { 
        let input = "".to_string();
        let output = Solution::letter_combinations(input);

        assert_eq!(output, Vec::<String>::new())
    }

    #[test]
    /// Input: digits = "2"
    /// Output: ["a","b","c"]
    
    fn example3() { 
        let input = "2".to_string();
        let output = Solution::letter_combinations(input);
        let example_output = Vec::from(["a", "b", "c"].map(String::from));
        assert_eq!(output, example_output);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 0 <= digits.length <= 4
    fn digits_len_max() { 
        let input = "23456".to_string();
        let _output = Solution::letter_combinations(input);
    }

    #[test]
    #[should_panic]
    /// digits[i] is a digit in the range ['2', '9']
    fn digits_val_min() { 
        let input = "123".to_string();
        let _output = Solution::letter_combinations(input);
    }

    #[test]
    #[should_panic]
    /// digits[i] is a digit in the range ['2', '9']
    fn digits_val_zero() { 
        let input = "110".to_string();
        let _output = Solution::letter_combinations(input);
    }

    #[test]
    #[should_panic]
    // digits[i] does not contain the pound key
    fn digits_val_pound() { 
        let input = "1#0".to_string();
        let _output = Solution::letter_combinations(input);
    }

    #[test]
    #[should_panic]
    // digits[i] does not contain the star key
    fn digits_val_star() { 
        let input = "1*0".to_string();
        let _output = Solution::letter_combinations(input);
    }
}
