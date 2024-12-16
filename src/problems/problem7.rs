pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let positive: bool = x.is_positive();
        let x_rev_string: String = x.abs().to_string().chars().rev().collect();
        let x_rev_int: i32 = x_rev_string.parse::<i32>().unwrap_or(0);
        match positive {
            true => x_rev_int,
            false => x_rev_int * -1,
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: x = 123
    /// Output: 321
    fn example1() {
        let input = 123;
        let output = Solution::reverse(input);
        assert_eq!(output, 321);
    }

    #[test]
    /// Example 2:
    /// Input: x = -123
    /// Output: -321
    fn example2() {
        let input = -123;
        let output = Solution::reverse(input);
        assert_eq!(output, -321);
    }

    #[test]
    /// Example 3:
    /// Input: x = 120
    /// Output: 21
    fn example3() {
        let input = 120;
        let output = Solution::reverse(input);
        assert_eq!(output, 21);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    //  -2^31 <= x <= 2^31 - 1
    // No need to test, constraint enforced by integer bounds
}
