#[allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        assert!(nums1.len() <= 1000 && nums2.len() <= 1000);
        assert!(nums1.len() > 0 || nums2.len() > 0);

        let mut nums_both = nums1.clone();
        nums_both.extend(&nums2);
        nums_both.sort();

        for num in nums_both.iter() {
            assert!(num <= &1_000_000)
        }

        let nums_both_len = nums_both.len();
        match nums_both_len % 2 == 0 {
            true => {
                let even_median: (i32, i32) = (
                    nums_both[nums_both_len / 2 - 1],
                    nums_both[nums_both_len / 2],
                );
                (even_median.0 + even_median.1) as f64 / 2.0
            }
            false => nums_both[nums_both_len / 2] as f64,
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// ### Example 1:
    /// Input: nums1 = [1,3], nums2 = [2]
    /// Output: 2.00000
    /// Explanation: merged array = [1,2,3] and median is 2.
    fn example1() {
        let input1 = vec![1, 3];
        let input2 = vec![2];
        let output = Solution::find_median_sorted_arrays(input1, input2);
        assert_eq!(output, 2.00000);
    }

    #[test]
    /// ### Example 2:
    /// Input: nums1 = [1,2], nums2 = [3,4]
    /// Output: 2.50000
    /// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
    fn example2() {
        let input1 = vec![1, 2];
        let input2 = vec![3, 4];
        let output = Solution::find_median_sorted_arrays(input1, input2);
        assert_eq!(output, 2.50000);
    }

    #[test]
    /// Tricky Case, one empty, one single value
    fn tricky1() {
        let input1 = vec![];
        let input2 = vec![1];
        let output = Solution::find_median_sorted_arrays(input1, input2);
        assert_eq!(output, 1.00000);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 0 <= m <= 1000
    fn num1_len_max() {
        let input1 = vec![1; 1001];
        let input2 = vec![3, 4];
        let _output = Solution::find_median_sorted_arrays(input1, input2);
    }

    #[test]
    #[should_panic]
    /// 0 <= n <= 1000
    fn num2_len_max() {
        let input1 = vec![3, 4, 5];
        let input2 = vec![3; 1001];
        let _output = Solution::find_median_sorted_arrays(input1, input2);
    }

    #[test]
    #[should_panic]
    /// 1 <= m + n <= 2000
    fn both_inputs_not_empty() {
        let input1 = vec![];
        let input2 = vec![];
        let _output = Solution::find_median_sorted_arrays(input1, input2);
    }

    #[should_panic]
    /// -10^6 <= nums1[i], nums2[i] <= 10^6 -10^6 <= nums1[i], nums2[i] <= 10^6
    fn values_max_million() {
        let input1 = vec![1, 2, 1000001];
        let input2 = vec![3, 4];
        let _output = Solution::find_median_sorted_arrays(input1, input2);
    }
}
