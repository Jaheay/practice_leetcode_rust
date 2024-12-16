pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            panic!("String list cannot be empty");
        }
        
        if strs.len() > 200 {
            panic!("Strings must contain <= 200 elements");
        }
        
        if strs.iter().any(|s| s.len() > 200) {
            panic!("String element too long");
        }
        
        if strs.iter().any(|s| s.chars().any(|c| !c.is_alphabetic())) {
            panic!("String element contains non-alphabet character");
        }

        // Start with the first string as the "current prefix"
        // We do skip(1) because strs[0] is used as the initialization, and we don't want to search it.
        strs.iter().skip(1).fold(strs[0].clone(), |current_prefix, next_string| {
            // Compare the current prefix with the next string character by character
            current_prefix
                .chars() // Characters of the current prefix
                .zip(next_string.chars()) // Pair with characters of the next string
                .take_while(|(char_from_prefix, char_from_next)| char_from_prefix == char_from_next) // Only keep matching characters
                .map(|(matching_char, _)| matching_char) // Extract the matching characters
                .collect() // Collect the matching characters into a new string
        })

    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Input: strs = ["flower","flow","flight"]
    /// Output: "fl"
    fn example1() { 
        let input = Vec::from(["flower","flow","flight"].map(String::from));
        let output = Solution::longest_common_prefix(input);
        assert_eq!(output, "fl".to_string());
    }

    #[test]
    /// Input: strs = ["dog","racecar","car"]
    /// Output: ""
    /// Explanation: There is no common prefix among the input strings.
    fn example2() { 
        let input = Vec::from(["dog","racecar","car"].map(String::from));
        let output = Solution::longest_common_prefix(input);
        assert_eq!(output, "".to_string());
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 1 <= strs.length 
    fn vector_len_min() { 
        let input = vec![];
        let _output = Solution::longest_common_prefix(input);
    }
    
    #[test]
    #[should_panic]
    /// strs.length <= 200
    fn vector_len_max() { 
        let input = vec!["hello".to_string(); 201];
        let _output = Solution::longest_common_prefix(input);
    }
    
    #[test]
    #[should_panic]
    /// strs[i].length <= 200
    fn word_len_max() { 
        let input = vec!["hello".to_string(), "a".repeat(201), "world".to_string()];
        let _output = Solution::longest_common_prefix(input);
    }
    
    #[test]
    #[should_panic]
    /// strs[i] consists of only lowercase English letters.
    fn invalid_characters() { 
        let input = vec!["hello?".to_string(), "world!".to_string(), "foo >:)".to_string()];
        let _output = Solution::longest_common_prefix(input);
    }
}
