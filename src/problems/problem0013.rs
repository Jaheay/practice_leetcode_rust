pub struct Solution;

impl Solution {
    fn roman_value(roman: char) -> i32 {
        match roman {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        // Check length
        assert!(
            !s.is_empty() && s.len() <= 15,
            "Length of string is not within [1, 15]"
        );

        // Check if all characters are valid
        const VALID_CHARS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        for ch in s.chars() {
            assert!(
                VALID_CHARS.contains(&ch),
                "String contains invalid characters"
            );
        }

        let roman_sum = s
            .chars()
            .rev()
            .fold((0, 0), |(sum, previous_value), current_char| {
                let value = Self::roman_value(current_char);
                if value >= previous_value {
                    (sum + value, value)
                } else {
                    (sum - value, value)
                }
            });
        roman_sum.0 // Return sum from tuple
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// Constraint: 1 <= s.length <= 15`
    fn constraint_len() {
        let input: String = "X".repeat(16);
        let _output = Solution::roman_to_int(input);
    }

    #[test]
    #[should_panic]
    /// Constraint: `s` contains only the characters `('I', 'V', 'X', 'L', 'C', 'D', 'M')`.
    fn constaint_valid_char() {
        let input: String = "LoremIpsum".to_string();
        let _output = Solution::roman_to_int(input);
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    ///     Output: 3
    ///     Input: s = "III"
    ///     Explanation: III = 3.
    fn example1() {
        let input: String = "III".to_string();
        let output = Solution::roman_to_int(input);
        assert_eq!(output, 3);
    }

    #[test]
    /// Example 2:
    /// Input: s = "LVIII"
    /// Output: 58
    /// Explanation: L = 50, V= 5, III = 3.
    fn example2() {
        let input: String = "LVIII".to_string();
        let output = Solution::roman_to_int(input);
        assert_eq!(output, 58);
    }

    #[test]
    /// Example 3:
    /// Input: s = "MCMXCIV"
    /// Output: 1994
    /// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
    fn example3() {
        let input: String = "MCMXCIV".to_string();
        let output = Solution::roman_to_int(input);
        assert_eq!(output, 1994)
    }
}
