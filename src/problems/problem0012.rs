pub struct Solution;

impl Solution {
    pub fn int_to_roman(num_orig: i32) -> String {
        assert!((1..4000).contains(&num_orig));

        const ROMAN_MAPPING: [(&str, i32); 13] = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        let mut result = String::new();
        let mut num = num_orig;
        for &(roman_symbol, symbol_value) in ROMAN_MAPPING.iter() {
            while num >= symbol_value {
                result += roman_symbol;
                num -= symbol_value;
            }
        }

        result
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: num = 3
    /// Output: "III"
    /// Explanation: 3 is represented as 3 ones.
    fn example1() {
        let input: i32 = 3;
        let output = Solution::int_to_roman(input);
        assert_eq!(output, "III".to_string());
    }

    #[test]
    /// Example 2:
    /// Input: num = 58
    /// Output: "LVIII"
    /// Explanation: L = 50, V = 5, III = 3.
    fn example2() {
        let input: i32 = 58;
        let output = Solution::int_to_roman(input);
        assert_eq!(output, "LVIII".to_string());
    }

    #[test]
    /// Example 3:
    /// Input: num = 1994
    /// Output: "MCMXCIV"
    /// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
    fn example3() {
        let input: i32 = 1994;
        let output = Solution::int_to_roman(input);
        assert_eq!(output, "MCMXCIV".to_string());
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    // Constraint: num < 4000
    fn constaint_value_max() {
        let input: i32 = 9001;
        let _output = Solution::int_to_roman(input);
    }

    #[test]
    #[should_panic]
    // Constraint: 1 <= num
    fn constaint_value_min() {
        let input: i32 = 0;
        let _output = Solution::int_to_roman(input);
    }
}
