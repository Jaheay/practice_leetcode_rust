pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            panic!("Cannot divide by zero")
        }
        if divisor == dividend {
            return 1;
        }

        // dividend.checked_div(divisor).unwrap_or(i32::MAX)

        // Flip the signs
        let (mut dividend_abs, divisor_abs) = (dividend.unsigned_abs(), divisor.unsigned_abs());

        let mut quotient = 0;

        while dividend_abs >= divisor_abs {
            let mut shift = 0;

            // Find the largest multiple of divisor that fits into dividend
            while dividend_abs > (divisor_abs << (shift + 1)) {
                shift += 1;
            }
            // Add 2^shift to the quotient (this is how many times divisor fits)
            quotient += 1 << shift;
            // Subtract the largest multiple of divisor from dividend
            dividend_abs -= divisor_abs << shift;
        }

        let is_negative = dividend.is_negative() ^ divisor.is_negative();

        if quotient == 1 << 31 && !is_negative {
            return i32::MAX;
        }

        match is_negative {
            true => -quotient,
            false => quotient,
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Input: dividend = 10, divisor = 3
    /// Output: 3
    fn example_1() {
        let input_dividend = 10;
        let input_divisor = 3;
        let output = Solution::divide(input_dividend, input_divisor);
        assert_eq!(output, 3);
    }

    #[test]
    /// Input: dividend = 7, divisor = -3
    /// Output: -2
    fn example_2() {
        let input_dividend = 7;
        let input_divisor = -3;
        let output = Solution::divide(input_dividend, input_divisor);
        assert_eq!(output, -2);
    }

    #[test]
    /// Input: dividend = 2147483647, divisor = 1
    /// Output = 2147483647
    fn tricky1_max_div_by_one() {
        let input_dividend = 2147483647;
        let input_divisor = 1;
        let output = Solution::divide(input_dividend, input_divisor);
        assert_eq!(output, 2147483647)
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// divisor != 0
    fn divisor_zero() {
        let input_dividend = 10;
        let input_divisor = 0;
        let _output = Solution::divide(input_dividend, input_divisor);
    }
}
