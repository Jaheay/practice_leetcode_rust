use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn four_sum(mut numbers: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut valid_quadruplets = HashSet::<Vec<i32>>::new();

        // Validate constraints
        if target < -1e9 as i32 || target > 1e9 as i32 {
            panic!("Target should be within the range of -10^9 <= target <= 10^9");
        }

        if numbers.len() < 1 || numbers.len() > 200 {
            panic!("Array length should be within the range of 1 to 200");
        }

        if !numbers.iter().all(|&value| value >= -1e9 as i32 && value <= 1e9 as i32) {
            panic!("All values must be in the range of -1e9 <= value <= 1e9");
        }

        // If there are less than four values, return early.
        if numbers.len() < 4 {
            return vec![];
        }

        
        numbers.sort();
        let max_value = *numbers.last().unwrap(); // Largest number in the sorted array

        
        for (first_index, &first_value) in numbers.iter().enumerate() {

            // Skip duplicate first values to ensure unique quadruplets
            if first_index > 0 && first_value == numbers[first_index - 1] {
                continue;
            }
            
            // If the smallest possible sum using this first value exceeds the target, break early
            if target / 4 < first_value {
                break;
            }
            
            // Start the second loop from the next element after `first_index`
            for (second_index, &second_value) in numbers[first_index + 1..].iter().enumerate() {
                // Skip duplicate second values for unique quadruplets
                if second_index > 0 && second_value == numbers[first_index + 1 + second_index - 1] {
                    continue;
                }

                // If the smallest possible sum using these two values exceeds the target, break early
                if target.saturating_sub(first_value) / 3 < second_value {
                    break;
                }

                // Start the third loop from the next element after `first_index + 1 + second_index`
                for (third_index, &third_value) in numbers[first_index + 1 + second_index + 1..].iter().enumerate() {
                    
                    // Skip duplicate third values for unique quadruplets
                    if third_index > 0 && third_value == numbers[first_index + 1 + second_index + 1 + third_index - 1] {
                        continue;
                    }

                    // If the smallest possible sum using these three values exceeds the target, break early
                    if target
                        .saturating_sub(first_value)
                        .saturating_sub(second_value) / 2 < third_value
                    {
                        break;
                    }

                    // Calculate the required fourth value to achieve the target sum
                    let fourth_value = target
                        .saturating_sub(first_value)
                        .saturating_sub(second_value)
                        .saturating_sub(third_value);

                    // If the fourth value is out of bounds, skip
                    if fourth_value < third_value || max_value < fourth_value {
                        continue;
                    }

                    // Check if the required fourth value exists in the remaining slice
                    if numbers[first_index + 1 + second_index + 1 + third_index + 1..]
                        .binary_search(&fourth_value)
                        .is_ok()
                    {
                        valid_quadruplets.insert(vec![first_value, second_value, third_value, fourth_value]);
                    }
                }
            }
        }

        return valid_quadruplets.into_iter().collect();
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    fn all_quads_eq_target(valid_quads: Vec<Vec<i32>>, target: i32) -> bool {
        for quad in valid_quads {
            let quad_sum = quad[0] + quad[1] + quad[2] + quad[3];
            if quad_sum != target {
                return false;
            }
        }
        return true;
    }

    #[test]
    /// Input: nums = [1,0,-1,0,-2,2], target = 0
    /// Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
    fn example1() { 
        let input_nums = vec![1,0,-1,0,-2,2];
        let input_target = 0;
        let output = Solution::four_sum(input_nums, input_target);
        assert_eq!(output.len(), 3);
        assert!(all_quads_eq_target(output, input_target))
    }

    #[test]
    /// Input: nums = [2,2,2,2,2], target = 8
    /// Output: [[2,2,2,2]]
    fn example2() { 
        let input_nums = vec![2,2,2,2,2];
        let input_target = 8;
        let output = Solution::four_sum(input_nums, input_target);
        assert_eq!(output.len(), 1);
        assert!(all_quads_eq_target(output, input_target))
    }

}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 1 <= nums.length
    fn nums_len_min() {
        let input_nums = vec![];
        let input_target = 0;
        let _output = Solution::four_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// nums.length <= 200
    fn nums_len_max() {
        let input_nums = vec![2; 201];
        let input_target = 0;
        let _output = Solution::four_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// -1e9 <= nums[i] 
    fn nums_val_min() {
        let input_nums = vec![1, 2, 3, -1e9 as i32 - 1, 4, 5, 6];
        let input_target = 0;
        let _output = Solution::four_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// nums[i] <= 1e9
    fn nums_val_max() {
        let input_nums = vec![1, 2, 3, 1e9 as i32 + 1, 4, 5, 6];
        let input_target = 0;
        let _output = Solution::four_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// -1e9 <= target
    fn nums_target_min() {
        let input_nums = vec![1, -2, 3, -4, 5, -6, 7, -8];
        let input_target = -1e9 as i32 - 1;
        let _output = Solution::four_sum(input_nums, input_target);
    }

    #[test]
    #[should_panic]
    /// target <= 1e9
    fn nums_target_max() {
        let input_nums = vec![1, -2, 3, -4, 5, -6, 7, -8];
        let input_target = 1e9 as i32 + 1;
        let _output = Solution::four_sum(input_nums, input_target);
    }

}
