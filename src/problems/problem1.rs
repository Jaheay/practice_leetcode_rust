pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Ensure target value is acceptable
        assert!(target <= 1e9 as i32 && target >= -1e9 as i32);
        // Ensure nums length is acceptable
        assert!(nums.len() as i32 <= 1e4 as i32 && nums.len() >= 2);
        // Ensure each value of nums is less than 10e9
        for num in nums.iter() {
            assert!(((-1e9 as i32)..=(1e9 as i32)).contains(num));
        }

        /*
        /// Old Solution
        let mut result = vec![];

        for (index0, num) in nums.iter().enumerate() {
            let check = target - num;
            if let Some(index1) = nums.iter().position(|&i| i == check) {
                if index1 != index0 {
                    return result = vec![index0 as i32, index1 as i32];
                }
            }
        }
        */

        let mut index_map = HashMap::with_capacity(nums.len());

        for (index0, &num) in nums.iter().enumerate() {
            let check = target - num;
            if let Some(&index1) = index_map.get(&check) {
                return vec![index0 as i32, index1 as i32];
            } else {
                index_map.insert(num, index0);
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: nums = [2,7,11,15], target = 9
    /// Output: [0,1]
    /// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    fn example1() {
        let input_nums = vec![2, 7, 11, 15];
        let input_target = 9;
        let output = Solution::two_sum(input_nums, input_target);
        assert!(output == vec![0, 1] || output == vec![1, 0])
    }

    #[test]
    /// Example 2:
    /// Input: nums = [3,2,4], target = 6
    /// Output: [1,2]
    fn example2() {
        let input_nums = vec![3, 2, 4];
        let input_target = 6;
        let output = Solution::two_sum(input_nums, input_target);
        assert!(output == vec![1, 2] || output == vec![2, 1])
    }

    #[test]
    /// Example 3:
    /// Input: nums = [3,3], target = 6
    /// Output: [0,1]
    fn example3() {
        let input_nums = vec![3, 3];
        let input_target = 6;
        let output = Solution::two_sum(input_nums, input_target);
        assert!(output == vec![0, 1] || output == vec![1, 0])
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// `2 <= nums.length <= 10^4`
    fn constraint_nums_length_min() {
        let input_nums = vec![3];
        let input_target = 6;
        let _output = Solution::two_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// `2 <= nums.length <= 10^4`
    fn constraint_nums_length_max() {
        let input_nums = vec![3; 10001];
        let input_target = 6;
        let _output = Solution::two_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// `-10^9 <= nums[i] <= 10^9`
    fn constraint_nums_value_min() {
        let input_nums = vec![-1987654321, -1987654321, 3];
        let input_target = 6;
        let _output = Solution::two_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// `-10^9 <= nums[i] <= 10^9`
    fn constraint_nums_value_max() {
        let input_nums = vec![1987654321, 1987654321];
        let input_target = 6;
        let _output = Solution::two_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// `-10^9 <= target <= 10^9`
    fn constraint_target_min() {
        let input_nums = vec![3, 6, 9];
        let input_target = -1987654321;
        let _output = Solution::two_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// `-10^9 <= target <= 10^9`
    fn constraint_target_max() {
        let input_nums = vec![3, 6, 9];
        let input_target = 1987654321;
        let _output = Solution::two_sum(input_nums, input_target);
    }
}
