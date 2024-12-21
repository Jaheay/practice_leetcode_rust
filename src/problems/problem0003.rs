pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(mut s: String) -> i32 {
        let mut result = 0;
        let mut substr = String::new();
        assert!(s.len() <= 50000);

        while !s.is_empty() {
            let curr_ch = s.pop().expect("String is empty with len > 0");
            if let Some(curr_ch_pos) = substr.find(curr_ch) {
                if result < substr.len() {
                    result = substr.len();
                }
                substr.drain(..=curr_ch_pos);
            }
            substr.push(curr_ch);
        }

        if result < substr.len() {
            result = substr.len();
        }

        result as i32
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: s = "abcabcbb"
    /// Output: 3
    /// Explanation: The answer is "abc", with the length of 3.
    fn example1() {
        let input = "abcabcbb".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 3);
    }

    #[test]
    /// Example 2:
    /// Input: s = "bbbbb"
    /// Output: 1
    /// Explanation: The answer is "b", with the length of 1.
    fn example2() {
        let input = "bbbbb".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 1);
    }

    #[test]
    /// Example 3:
    /// Input: s = "pwwkew"
    /// Output: 3
    /// Explanation: The answer is "wke", with the length of 3.
    /// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
    fn example3() {
        let input = "pwwkew".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 3);
    }

    #[test]
    /// Tricky case, empty string,
    fn tricky1() {
        let input = "".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 0);
    }

    #[test]
    /// Tricky case, " " string.
    fn tricky2() {
        let input = " ".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 1);
    }

    #[test]
    /// Tricky case, abcb
    fn tricky3() {
        let input = "abcb".to_string();
        let output = Solution::length_of_longest_substring(input);
        assert_eq!(output, 3);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// `0 <= s.length <= 5 * 104`
    fn string_length_max() {
        let input: String = "abcde".repeat(10001);
        let _output = Solution::length_of_longest_substring(input);
    }
}
