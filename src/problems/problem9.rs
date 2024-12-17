pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let x_string = x.to_string();
        let x_chars: Vec<char> = x_string.chars().collect();
        let x_len = x_string.len();

        for radius in 0..(x_len / 2) {
            if x_chars[radius] != x_chars[(x_len - 1) - radius] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: x = 121
    /// Output: true
    /// Explanation: 121 reads as 121 from left to right and from right to left.
    fn example1() {
        let input = 121;
        let output = Solution::is_palindrome(input);
        assert!(output);
    }

    #[test]
    /// Example 2:
    /// Input: x = -121
    /// Output: false
    /// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
    fn example2() {
        let input = -121;
        let output = Solution::is_palindrome(input);
        assert!(!output);
    }

    #[test]
    /// Example 3:
    /// Input: x = 10
    /// Output: false
    /// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
    fn example3() {
        let input = 10;
        let output = Solution::is_palindrome(input);
        assert!(!output);
    }
}

/*
#[cfg(test)]
mod constraints {
    use super::*;

    //  -2^31 <= x <= 2^31 - 1
    // No need to test, constraint enforced by integer bounds
}
*/
