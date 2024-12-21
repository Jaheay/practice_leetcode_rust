pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        Self::validate_constraints(haystack.clone(), needle.clone());
        haystack.find(&needle).map_or(-1, |index| index as i32)
    }

    fn validate_constraints(haystack: String, needle: String) {
        if haystack.is_empty() || needle.is_empty() {
            panic!("Empty value provided")
        }

        if haystack.len() > 1e4 as usize || needle.len() > 1e4 as usize {
            panic!("Provided arguments too long")
        }

        if !haystack.chars().all(|c| c.is_ascii_lowercase()) {
            panic!("Haystack contains invalid characters")
        }

        if !needle.chars().all(|c| c.is_ascii_lowercase()) {
            panic!("Needle contains invalid characters")
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Input: haystack = "sadbutsad", needle = "sad"
    /// Output: 0
    fn example_1() {
        let input_haystack = "sadbutsad".to_string();
        let input_needle = "sad".to_string();
        let output_index = Solution::str_str(input_haystack, input_needle);
        assert_eq!(output_index, 0);
    }

    #[test]
    /// Input: haystack = "leetcode", needle = "leeto"
    /// Output: -1
    fn example_2() {
        let input_haystack = "leetcode".to_string();
        let input_needle = "leeto".to_string();
        let output_index = Solution::str_str(input_haystack, input_needle);
        assert_eq!(output_index, -1);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 1 <= haystack.length
    fn haystack_len_min() {
        let input_haystack = "".to_string();
        let input_needle = "a".to_string();
        let _output = Solution::str_str(input_haystack, input_needle);
    }

    #[test]
    #[should_panic]
    /// haystack.length <= 10^4
    fn haystack_len_max() {
        let input_haystack = "a".repeat(10_001);
        let input_needle = "b".to_string();
        let _output = Solution::str_str(input_haystack, input_needle);
    }

    #[test]
    #[should_panic]
    /// 1 <= needle.length
    fn needle_len_min() {
        let input_haystack = "a".to_string();
        let input_needle = "".to_string();
        let _output = Solution::str_str(input_haystack, input_needle);
    }

    #[test]
    #[should_panic]
    /// needle.length <= 10^4
    fn needle_len_max() {
        let input_haystack = "b".to_string();
        let input_needle = "a".repeat(10_001);
        let _output = Solution::str_str(input_haystack, input_needle);
    }

    #[test]
    #[should_panic]
    /// haystack and needle consist of only lowercase English characters.
    fn haystack_invalid_chars() {
        let input_haystack = "helloWORLD".to_string();
        let input_needle = "hello".to_string();
        let _output = Solution::str_str(input_haystack, input_needle);
    }

    #[test]
    #[should_panic]
    /// haystack and needle consist of only lowercase English characters.
    fn needle_invalid_chars() {
        let input_haystack = "hello".to_string();
        let input_needle = "HELLO".to_string();
        let _output = Solution::str_str(input_haystack, input_needle);
    }
}
