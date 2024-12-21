pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        if !(1..=100).contains(&nums.len()) {
            panic!("Length of nums out of range [1,100]");
        }

        if nums.iter().any(|num| !(0..=100).contains(num)) {
            panic!("Value in nums vector is out of range [0,100]");
        }

        let len = nums.len();
        if len < 2 {
            return;
        }

        // Find the break point from the right
        let break_point = nums
            .windows(2)
            .rposition(|pair| pair[0] < pair[1])
            .map(|index| index + 1);

        // If no break point is found, reverse the entire array
        if break_point.is_none() {
            nums.reverse();
            return;
        }
        // Find the smallest element larger than the pivot, to the right of the pivot
        let break_point = break_point.unwrap();
        let pivot_index = break_point - 1;
        let swap_index = nums[break_point..]
            .iter()
            .rposition(|&num| num > nums[pivot_index])
            .map(|index| index + break_point)
            .unwrap();

        // Swap the Values & Reverse the subarray to the right of the pivot
        nums.swap(pivot_index, swap_index);
        nums[break_point..].reverse();
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Input: nums = [1,2,3]
    /// Output: [1,3,2]
    fn example_1() {
        let mut input_nums = vec![1, 2, 3];
        Solution::next_permutation(&mut input_nums);
        assert_eq!(input_nums, vec![1, 3, 2]);
    }

    #[test]
    /// Input: nums = [3,2,1]
    /// Output: [1,2,3]
    fn example_2() {
        let mut input_nums = vec![3, 2, 1];
        Solution::next_permutation(&mut input_nums);
        assert_eq!(input_nums, vec![1, 2, 3]);
    }

    #[test]
    /// Input: nums = [1,1,5]
    /// Output: [1,5,1]
    fn example_3() {
        let mut input_nums = vec![1, 1, 5];
        Solution::next_permutation(&mut input_nums);
        assert_eq!(input_nums, vec![1, 5, 1]);
    }

    #[test]
    /// Input: nums = [1,1,5]
    /// Output: [1,5,1]
    fn tricky1_width_2() {
        let mut input_nums = vec![1, 2];
        Solution::next_permutation(&mut input_nums);
        assert_eq!(input_nums, vec![2, 1]);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 1 <= nums.length <= 100
    fn num_len_min() {
        let mut input_nums = vec![];
        Solution::next_permutation(&mut input_nums);
    }

    #[test]
    #[should_panic]
    /// 1 <= nums.length <= 100
    fn num_len_max() {
        let mut input_nums = vec![1; 101];
        Solution::next_permutation(&mut input_nums);
    }

    #[test]
    #[should_panic]
    /// 0 <= nums[i] <= 100
    fn num_val_min() {
        let mut input_nums = vec![-1, 0, 1];
        Solution::next_permutation(&mut input_nums);
    }

    #[test]
    #[should_panic]
    /// 0 <= nums[i] <= 100
    fn num_val_max() {
        let mut input_nums = vec![50, 101];
        Solution::next_permutation(&mut input_nums);
    }
}
