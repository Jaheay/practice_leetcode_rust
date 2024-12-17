pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        
        if s.len() > 1e4 as usize {
            panic!("String too long")
        }

        for character in s.chars() {
            match character {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                    if stack.pop() != Some(character) {
                        return false;
                    }
                }
                _ => panic!("Invalid characters in input"), // Given input consists of only valid parentheses
            }
        }
        
        stack.is_empty()
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: s = "()"
    /// Output: true
    fn example1() {
        let input = "()".to_string();
        let output = Solution::is_valid(input);
        assert_eq!(output, true);
    }

    #[test]
    /// Example 2:
    /// Input: s = "()[]{}"
    /// Output: true
    fn example2() {
        let input = "()[]{}".to_string();
        let output = Solution::is_valid(input);
        assert_eq!(output, true);
    }

    #[test]
    /// Example 3:
    /// Input: s = "(]"
    /// Output: false
    fn example3() {
        let input = "(]".to_string();
        let output = Solution::is_valid(input);
        assert_eq!(output, false);
    }

    #[test]
    /// Example 4:
    /// Input: s = "([])"
    /// Output: true
    fn example4() {
        let input = "([])".to_string();
        let output = Solution::is_valid(input);
        assert_eq!(output, true);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 1 <= s.length <= 1e4
    fn string_length_max() {
        let input = "()".repeat(5001); // 5001 pairs = 10002 characters
        let _output = Solution::is_valid(input);
    }

    #[test]
    #[should_panic]
    /// s consists of parentheses only '()[]{}'.
    fn string_invalid() {
        let input = "()<>[]".to_string(); 
        let _output = Solution::is_valid(input);
    }
}