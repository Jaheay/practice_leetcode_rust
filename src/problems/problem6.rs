pub struct Solution;

use ZigZagState::*;
enum ZigZagState {
    Zig,
    Zag,
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        assert!(num_rows >= 1 && num_rows <= 1000);
        assert!(s.len() >= 1 && s.len() <= 1000);
        for ch in s.chars() {
            assert!(ch.is_alphabetic() || ch == '.' || ch == ',');
        }

        // Special Case, 1 row
        if num_rows < 2 {
            return s;
        }

        let mut rows = vec![String::from(""); num_rows as usize];
        let mut row: i32 = 0;
        let mut state: ZigZagState = Zig;

        for ch in s.chars() {
            rows[row as usize].push(ch);

            row += match state {
                Zig => 1,
                Zag => -1,
            };

            state = if row > 0 && row < (num_rows - 1) {
                state
            } else {
                match state {
                    Zig => Zag,
                    Zag => Zig,
                }
            };
        }

        rows.concat()
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// ### Example 1:
    /// Input: s = "PAYPALISHIRING", numRows = 3
    /// Output: "PAHNAPLSIIGYIR"
    fn example1() {
        let input = "PAYPALISHIRING".to_string();
        let numrows = 3;
        let output = Solution::convert(input, numrows);
        assert_eq!(output, "PAHNAPLSIIGYIR".to_string());
    }

    #[test]
    /// ### Example 2:
    /// Input s = "PAYPALISHIRING", numRows = 4
    /// Output "PINALSIGYAHRPI"
    /// Explanation:
    /// P     I    N
    /// A   L S  I G
    /// Y A   H R
    /// P     I
    fn example2() {
        let input = "PAYPALISHIRING".to_string();
        let numrows = 4;
        let output = Solution::convert(input, numrows);
        assert_eq!(output, "PINALSIGYAHRPI".to_string());
    }

    #[test]
    /// ### Example 3:
    /// Input: s = "A", numRows = 1
    /// Output: "A"
    fn example3() {
        let input = "A".to_string();
        let numrows = 1;
        let output = Solution::convert(input, numrows);
        assert_eq!(output, "A".to_string());
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    // `1 <= s.length <= 1000`
    fn s_len_min() {
        let input = "".to_string();
        let numrows = 1;
        let _output = Solution::convert(input, numrows);
    }
    #[test]
    #[should_panic]
    // `1 <= s.length <= 1000`
    fn s_len_max() {
        let input = "A".repeat(1001);
        let numrows = 1;
        let _output = Solution::convert(input, numrows);
    }

    #[test]
    #[should_panic]
    // `s` consists of English letters (lower-case and upper-case), `','` and `'.'`.
    fn invalid_chars() {
        let input = "AA%^AA".to_string();
        let numrows = 1;
        let _output = Solution::convert(input, numrows);
    }

    #[test]
    #[should_panic]
    //  `1 <= numRows <= 1000`
    fn numrows_min() {
        let input = "AAAAA".to_string();
        let numrows = 0;
        let _output = Solution::convert(input, numrows);
    }

    #[test]
    #[should_panic]
    //  `1 <= numRows <= 1000`
    fn numrows_max() {
        let input = "AAAAA".to_string();
        let numrows = 1001;
        let _output = Solution::convert(input, numrows);
    }
}
