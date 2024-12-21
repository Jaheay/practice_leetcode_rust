pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n_max: i32) -> Vec<String> {
        assert!(n_max > 0 && n_max <= 1e4 as i32);

        let mut fb_vec: Vec<String> = (1..=n_max)
            .map(|n| match (n % 3, n % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                _ => n.to_string(),
            })
            .collect();
        fb_vec
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: n = 3
    /// Output: ["1","2","Fizz"]
    fn example1() {
        let input = 3;
        let output = Solution::fizz_buzz(input);
        assert_eq!(output, vec!["1", "2", "Fizz"])
    }

    #[test]
    /// Example 2:
    /// Input: n = 5
    /// Output: ["1","2","Fizz","4","Buzz"]
    fn example2() {
        let input = 5;
        let output = Solution::fizz_buzz(5);
        assert_eq!(output, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    /// Example 3:
    /// Input: n = 15
    /// Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
    fn example3() {
        let input = 15;
        let output = Solution::fizz_buzz(15);
        assert_eq!(
            output,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 1 <= n <= 1e4
    fn min_val() {
        let input = -1;
        let _output = Solution::fizz_buzz(input);
    }

    #[test]
    #[should_panic]
    /// 1 <= n <= 1e4
    fn max_val() {
        let input = 1e4 as i32 + 1;
        let _output = Solution::fizz_buzz(input);
    }
}
