pub struct Solution;

impl Solution {
    pub fn longest_palindrome(mut s: String) -> String {
        assert!(s.chars().all(|ch| ch.is_alphanumeric()));
        assert!(s.len() > 0 && s.len() <= 1000);

        let s_len = s.len();

        // Special Case, String Length of 1
        if s_len <= 1 {
            return s;
        }

        // To detect odd palindromes, we need to insert a dummy char between each
        // character, and at the start/end of the string.
        const DUMMY_CH: char = '#';
        let mut s_dummy: Vec<char> = Vec::with_capacity(2 * s_len + 1);
        for ch in s.chars() {
            s_dummy.push(DUMMY_CH); // # between each char
            s_dummy.push(ch);
        }
        s_dummy.push(DUMMY_CH); // Last char $
        let s_dummy_len = s_dummy.len();

        // Store Palindrome Radius from Centre
        let mut palindrome_len: Vec<usize> = vec![0; s_dummy_len];
        let mut current_centre: usize = 0;
        let mut right_bound: usize = 0;

        // Take s_dummy character by character, updating current_centre only when we we
        // need to evaluate a palindrome
        for chk_idx in 0..s_dummy_len {
            if chk_idx < right_bound && chk_idx > current_centre {
                // If were inside a palindrome, simply mirror the value
                palindrome_len[chk_idx] = std::cmp::min(
                    right_bound - chk_idx,
                    palindrome_len[2 * current_centre - chk_idx],
                );

                // If we excede the rihgt bound, move the current checking index (ch)
                if chk_idx + palindrome_len[chk_idx] >= right_bound {
                    current_centre = chk_idx;
                    right_bound = chk_idx + palindrome_len[chk_idx];

                    // If right_bound excedes the string length, break;
                    if right_bound >= s_dummy_len - 1 {
                        break;
                    }
                } else {
                    continue;
                }
            }

            let mut radius: usize = (palindrome_len[chk_idx].saturating_sub(1)) / 2;
            radius += 1;

            while chk_idx >= radius
                && chk_idx + radius <= s_dummy_len - 1
                && s_dummy[chk_idx - radius] == s_dummy[chk_idx + radius]
            {
                // +2 since were storing length, not radius
                palindrome_len[chk_idx] = palindrome_len[chk_idx] + 2;
                radius += 1;
            }
        }

        let centre_of_max = palindrome_len
            .iter()
            .enumerate()
            .max_by_key(|(_, &val)| val)
            .map(|(idx, _)| idx)
            .expect("No Maximum Found");
        let radius_of_max = (palindrome_len[centre_of_max].saturating_sub(1)) / 2;
        let mut result = &s_dummy
            [(centre_of_max - radius_of_max)..(centre_of_max + radius_of_max + 1)]
            .into_iter()
            .collect::<String>();
        println!(
            "s_dummy: {:?}, com: {}, lcom: {}, rom: {}, result: {}",
            s_dummy, centre_of_max, palindrome_len[centre_of_max], radius_of_max, result
        );
        result.replace(DUMMY_CH, "") // Remove the dummy chars
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: s = "babad"
    /// Output: "bab"
    /// Explanation: "aba" is also a valid answer.
    fn example1() {
        let input = "babad".to_string();
        let output = Solution::longest_palindrome(input);
        println!("{}", output);
        assert!(output == "aba".to_string() || output == "bab".to_string());
    }

    #[test]
    /// Example 2:
    /// Input: s = "cbbd"
    /// Output: "bb"
    fn example2() {
        let input = "cbbd".to_string();
        let output = Solution::longest_palindrome(input);
        assert_eq!(output, "bb".to_string());
    }

    #[test]
    // Tricky Example, Single Char
    fn tricky1() {
        let input = "a".to_string();
        let output = Solution::longest_palindrome(input);
        assert_eq!(output, "a".to_string());
    }

    #[test]
    // Tricky Case, no palindrome
    fn tricky2() {
        let input = "ac".to_string();
        let output = Solution::longest_palindrome(input);
        assert_eq!(output, "c".to_string());
    }
}
#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    // 1 <= s.length <= 1000
    fn string_len_min() {
        let input = "".to_string();
        let _output = Solution::longest_palindrome(input);
    }

    #[test]
    #[should_panic]
    // 1 <= s.length <= 1000
    fn string_len_max() {
        let input = "a".repeat(1001).to_string();
        let _output = Solution::longest_palindrome(input);
    }

    #[test]
    #[should_panic]
    // s consist of only digits and English letters.
    fn string_is_alphanumeric() {
        let input = "*&^%$#".to_string();
        let _output = Solution::longest_palindrome(input);
    }
}
