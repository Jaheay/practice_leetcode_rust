#![allow(clippy::ptr_arg)]

pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        Self::validate_constraints(nums, val);

        // nums.retain(|&num| num != val);
        // nums.len() as i32

        let mut write_index = 0;
        for read_index in 0..nums.len() {
            if nums[read_index] != val {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }
        write_index as i32
    }

    fn validate_constraints(nums: &[i32], val: i32) {
        if nums.len() > 100 {
            panic!(
                "The length of the provided numbers ({}) is outside the allowed range [0, 100]",
                nums.len()
            );
        }

        if !(0..=100).contains(&val) {
            panic!("Value outside of the allowed range of [0, 100]");
        }

        if nums.iter().any(|&num| !(0..=50).contains(&num)) {
            panic!("Value outside of the allowed range of [0, 50]");
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Input: nums = [3,2,2,3], val = 3
    /// Output: 2, nums = [2,2,_,_]
    fn example_1() {
        let mut input_nums = vec![3, 2, 2, 3];
        let input_val = 3;
        let output = Solution::remove_element(&mut input_nums, input_val);
        assert_eq!(output, 2);
        assert!(input_nums[..output as usize]
            .iter()
            .all(|&x| x != input_val));
    }

    #[test]
    /// Input: nums = [0,1,2,2,3,0,4,2], val = 2
    /// Output: 5, nums = [0,1,4,0,3,_,_,_]
    fn example_2() {
        let mut input_nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let input_val = 2;
        let output = Solution::remove_element(&mut input_nums, input_val);
        assert_eq!(output, 5);
        assert!(input_nums[..output as usize]
            .iter()
            .all(|&x| x != input_val));
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 0 <= nums.length <= 100
    fn num_len_max() {
        let mut input_nums = vec![5; 101];
        let input_val = 0;
        let _output = Solution::remove_element(&mut input_nums, input_val);
    }

    #[test]
    #[should_panic]
    /// 0 <= nums[i] <= 50
    fn num_val_min() {
        let mut input_nums = vec![-1, 0, 1];
        let input_val = 0;
        let _output = Solution::remove_element(&mut input_nums, input_val);
    }

    #[test]
    #[should_panic]
    /// 0 <= nums[i] <= 50
    fn num_val_max() {
        let mut input_nums = vec![50, 51, 52];
        let input_val = 50;
        let _output = Solution::remove_element(&mut input_nums, input_val);
    }

    #[test]
    #[should_panic]
    /// 0 <= val <= 100
    fn val_min() {
        let mut input_nums = vec![1, 2, 3];
        let input_val = -1;
        let _output = Solution::remove_element(&mut input_nums, input_val);
    }

    #[test]
    #[should_panic]
    /// 0 <= val <= 100
    fn val_max() {
        let mut input_nums = vec![1, 2, 3];
        let input_val = 101;
        let _output = Solution::remove_element(&mut input_nums, input_val);
    }
}
