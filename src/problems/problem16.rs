use std::cmp::Ordering;
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut closest_sum: Option<i32> = None;

        if target < -1e4 as i32 || target > 1e4 as i32 {
            panic!("Target value must be in the range -10^4 to 10^4 inclusive")
        }

        if nums.len() < 3 || nums.len() > 500 {
            panic!("Nums must have between 3 and 500 values")
        }

        if !nums.iter().all(|&val| val >= -1000 as i32 && val <= 1000 as i32) {
            panic!("All values must be in the range -1000 <= value <= 1000");
        }

        for (index, &val) in nums.iter().enumerate() { 

            let mut left = index + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let current_sum = val + nums[left] + nums[right];

                match current_sum.cmp(&target) {
                    Ordering::Less => left +=1,
                    Ordering::Greater => right -= 1,
                    Ordering::Equal => return target
                }
                
                match closest_sum {
                    None => closest_sum = Some(current_sum),
                    Some(current_closest) => {
                            if (target - current_sum).abs() < (target - current_closest).abs() {
                                closest_sum = Some(current_sum);
                            }
                    }
                }
            }
        }
    
        closest_sum.unwrap()
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Input: nums = [-1,2,1,-4], target = 1
    /// Output: 2
    /// Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
    fn example1() { 
        let input_nums = vec![-1,2,1,-4];
        let input_target = 1;
        let output = Solution::three_sum_closest(input_nums, input_target);
        assert_eq!(output, 2);
    }

    #[test]
    /// Input: nums = [0,0,0], target = 1
    /// Output: 0
    /// Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
    fn example2() { 
        let input_nums = vec![0, 0, 0];
        let input_target = 0;
        let output = Solution::three_sum_closest(input_nums, input_target);
        assert_eq!(output, 0)
    }
}

#[cfg(test)]
mod constraints {
    use super::*;


    #[test]
    #[should_panic]
    /// 3 <= nums.length <= 500

    fn nums_length_min() { 
        let input_nums = vec![1,2];
        let input_target = 5;
        let _output = Solution::three_sum_closest(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// 3 <= nums.length <= 500
    fn nums_length_max() { 
        let input_nums = vec![5; 501];
        let input_target = 4;
        let _output = Solution::three_sum_closest(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// -1000 <= nums[i] <= 1000
    fn nums_value_min() { 
        let input_nums = vec![1, 2, -1001, 3, 4];
        let input_target = 5;
        let _output = Solution::three_sum_closest(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// -1000 <= nums[i] <= 1000
    fn nums_value_max() { 
        let input_nums = vec![1, 2, 1001, 3, 4];
        let input_target = 5;
        let _output = Solution::three_sum_closest(input_nums, input_target);
    }
    
    #[test]
    #[should_panic]
    /// -104 <= target <= 1e4
    fn target_min() {
        let input_nums = vec![1, 2, 3, 4, 5];
        let input_target = -1e4 as i32 - 1;
        let _output = Solution::three_sum_closest(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// -104 <= target <= 1e4
    fn target_max() { 
        let input_nums = vec![1, 2, 3, 4, 5];
        let input_target = 1e4 as i32 + 1;
        let output = Solution::three_sum_closest(input_nums, input_target);
    }
}
