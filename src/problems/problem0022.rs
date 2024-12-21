pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if !(1..=8).contains(&n) {
            panic!("Input value must be in the range of [1, 8]");
        }

        let mut valid_combos = Vec::new();
        let mut current_string = String::new();
        Self::generate(n as usize, 0, 0, &mut current_string, &mut valid_combos);

        valid_combos
    }

    fn generate(
        n: usize,
        open: usize,
        close: usize,
        current_string: &mut String,
        valid_combos: &mut Vec<String>,
    ) {
        if current_string.len() == n * 2 {
            valid_combos.push(current_string.clone());
            return;
        }

        if open < n {
            current_string.push('(');
            Self::generate(n, open + 1, close, current_string, valid_combos);
            current_string.pop();
        }

        if close < open {
            current_string.push(')');
            Self::generate(n, open, close + 1, current_string, valid_combos);
            current_string.pop();
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: n = 3
    /// Output: ["((()))","(()())","(())()","()(())","()()()"]
    fn example1() {
        let input = 3;
        let output = Solution::generate_parenthesis(input);
        let example_output =
            Vec::from(["((()))", "(()())", "(())()", "()(())", "()()()"].map(String::from));
        assert_eq!(output, example_output);
    }

    #[test]
    /// Example 2:
    /// Input: n = 1
    /// Output: ["()"]
    fn example2() {
        let input = 1;
        let output = Solution::generate_parenthesis(input);
        let example_output = Vec::from(["()"].map(String::from));
        assert_eq!(output, example_output)
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 1 <= n
    fn n_min() {
        let input = 0;
        let _output = Solution::generate_parenthesis(input);
    }

    #[test]
    #[should_panic]
    /// n <= 8
    fn n_max() {
        let input = 9;
        let _output = Solution::generate_parenthesis(input);
    }
}
