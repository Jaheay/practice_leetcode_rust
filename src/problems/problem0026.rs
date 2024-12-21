#![allow(clippy::ptr_arg)]

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::validate_constraints(nums);

        // nums.dedup();
        // nums.len() as i32

        let mut unique_position = 0;

        for current_position in 1..nums.len() {
            if nums[current_position] != nums[unique_position] {
                unique_position += 1;
                nums[unique_position] = nums[current_position];
            }
        }
        nums.truncate(unique_position + 1);

        (unique_position + 1) as i32
    }

    fn validate_constraints(nums: &[i32]) {
        if !(1..=3 * 1e4 as usize).contains(&nums.len()) {
            panic!(
                "The length of the provided numbers ({}) is outside the allowed range [1, 30000]",
                nums.len()
            );
        }

        if nums.iter().any(|&num| !(-100..=100).contains(&num)) {
            panic!("Value outside of the allowed range of [-100, 100]");
        }

        if nums.windows(2).any(|w| w[0] > w[1]) {
            panic!("Input is not sorted in non-decreasing order");
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Input: nums = [1,1,2]
    /// Output: 2, nums = [1,2,_]
    fn example1() {
        let mut input = vec![1, 1, 2];
        let output = Solution::remove_duplicates(&mut input);
        assert_eq!(output, 2);
        assert_eq!(input, vec![1, 2]);
    }

    #[test]
    /// Input: nums = [0,0,1,1,1,2,2,3,3,4]
    /// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
    fn example2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let output = Solution::remove_duplicates(&mut input);
        assert_eq!(output, 5);
        assert_eq!(input, vec![0, 1, 2, 3, 4]);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 1 <= nums.length <= 3 * 1e4
    fn num_len_min() {
        let mut input = vec![];
        let _output = Solution::remove_duplicates(&mut input);
    }

    #[test]
    #[should_panic]
    /// 1 <= nums.length <= 3 * 104
    fn num_len_max() {
        let mut input = vec![5; (3.0 * 1e4 + 1.0) as usize];
        let _output = Solution::remove_duplicates(&mut input);
    }

    #[test]
    #[should_panic]
    /// -100 <= nums[i] <= 100
    fn num_val_min() {
        let mut input = vec![-101, -100, -99];
        let _output = Solution::remove_duplicates(&mut input);
    }

    #[test]
    #[should_panic]
    /// -100 <= nums[i] <= 100
    fn num_val_max() {
        let mut input = vec![99, 100, 101];
        let _output = Solution::remove_duplicates(&mut input);
    }

    #[test]
    #[should_panic]
    /// nums is sorted in non-decreasing order.
    fn num_sort_decending() {
        let mut input = vec![5, 4, 3, 2, 1];
        let _output = Solution::remove_duplicates(&mut input);
    }
}
