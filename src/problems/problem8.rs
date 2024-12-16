pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        const VALID_SYMBOLS: &str = " +-.";
        assert!(s.len() <= 200);
        for ch in s.chars() {
            assert!(ch.is_alphanumeric() || VALID_SYMBOLS.contains(ch))
        }

        let s_trimmed = s.trim_start();
        println!("{}", s_trimmed);

        let (s_abs, is_neg) = match s_trimmed.strip_prefix('-') {
            Some(s) => (s, true),
            None => (s_trimmed.strip_prefix('+').unwrap_or(s_trimmed), false),
        };
        println!("s: {}, neg: {:?}", s_abs, is_neg);

        let mut result = String::new();

        for ch in s_abs.chars() {
            match ch {
                '0'..='9' => result.push(ch),
                _ => break,
            }
        }

        println!("{}", result);

        if result.is_empty() {
            return 0;
        }

        if is_neg {
            result.insert_str(0, "-");
        }

        match result.parse::<i32>() {
            Ok(result) => result,
            Err(err) => match err.kind() {
                std::num::IntErrorKind::PosOverflow => i32::MAX,
                std::num::IntErrorKind::NegOverflow => i32::MIN,
                _ => 0,
            },
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: s = "42"
    /// Output: 42
    fn example1() {
        let input = "42".to_string();
        let output = Solution::my_atoi(input);
        assert_eq!(output, 42);
    }

    #[test]
    /// Example 2:
    /// Input: s = "   -42"
    /// Output: -42
    fn example2() {
        let input = "   -42".to_string();
        let output = Solution::my_atoi(input);
        assert_eq!(output, -42);
    }

    #[test]
    /// Example 3:
    /// Input: s = "4193 with words"
    /// Output = 4193
    fn example3() {
        let input = "4193 with words".to_string();
        let output = Solution::my_atoi(input);
        assert_eq!(output, 4193);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 0 <= s.length <= 200
    fn s_len_max() {
        let input = "A".repeat(201);
        let _output = Solution::my_atoi(input);
    }

    #[test]
    #[should_panic]
    /// `s` consists of English letters (lower-case and upper-case), digits (`0-9`), `' '`, `'+'`, `'-'`, and `'.'`
    fn invalid_chars() {
        let input = "  hdh%$".to_string();
        let _output = Solution::my_atoi(input);
    }
}
