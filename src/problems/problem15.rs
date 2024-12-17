use std::{cmp::Ordering, collections::HashSet};

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut valid_triples = HashSet::<Vec<i32>>::new();

        // Verify constraints
        if nums.len() < 3 || nums.len() > 3000 {
            panic!("Array length should be between 3 and 3000 inclusive");
        }

        if !nums.iter().all(|&val| val >= -1e5 as i32 && val <= 1e5 as i32) {
            panic!("All values must be in the range -1e5 <= value <= 1e5");
        }

        // Sort input array
        nums.sort();

        for (index, &val) in nums.iter().enumerate() { 


            let mut left = index + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = val + nums[left] + nums[right];

                match sum.cmp(&0) { 
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                    Ordering::Equal =>
                    {
                        let valid_triple = vec![val, nums[left], nums[right]];
                        valid_triples.insert(valid_triple);
                        left += 1;
                        right -= 1;
                        
                    }
                }
            }
        }

        return valid_triples.into_iter().collect();
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    fn all_triples_eq_zero(valid_triples: Vec<Vec<i32>>) -> bool {
        for triple in valid_triples {
            let triple_sum = triple[0] + triple[1] + triple[2];
            if triple_sum != 0 {
                return false;
            }
        }
        return true;
    }

    #[test]
    /// Input: nums = [-1,0,1,2,-1,-4]
    /// Output: [[-1,-1,2],[-1,0,1]]
    /// Explanation: 
    /// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
    /// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
    /// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
    /// The distinct triplets are [-1,0,1] and [-1,-1,2].
    /// Notice that the order of the output and the order of the triplets does not matter.
    fn example1() { 
        let input = vec![-1,0,1,2,-1,-4];
        let output = Solution::three_sum(input);

        assert_eq!(output.len(), 2);

        assert!(all_triples_eq_zero(output))
    }

    #[test]
    /// Input: nums = [0,1,1]
    /// Output: []
    /// Explanation: The only possible triplet does not sum up to 0.
    fn example2() {
        let input = vec![0,1,1];
        let output = Solution::three_sum(input);
        assert_eq!(output, Vec::<Vec<i32>>::new());
    }

    #[test]
    /// Input: nums = [0,0,0]
    /// Output: [[0,0,0]]
    /// Explanation: The only possible triplet sums up to 0.
    fn example3() { 
        let input = vec![0,0,0];
        let output = Solution::three_sum(input);
        assert_eq!(output, vec![vec![0,0,0]])
    }

    #[test]
    fn test1() {
        let input = vec![1,-1,-1,0];
        let output = Solution::three_sum(input);
        assert_eq!(output, vec![vec![-1,0,1]]);
    }

    #[test]
    fn test2() {
        let input = vec![3,0,-2,-1,1,2];
        let output = Solution::three_sum(input);
        assert_eq!(output.len(), 3);
        
        assert!(all_triples_eq_zero(output))
        
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 3 <= nums.length
    fn nums_length_min() { 
        let input = vec![1,2];
        let _output = Solution::three_sum(input);
    }

    #[test]
    #[should_panic]
    /// nums.length <= 3000
    fn nums_length_max() { 
        let input = vec![3; 3001];
        let _output = Solution::three_sum(input);
    }

    #[test]
    #[should_panic]
    /// -1e5 <= nums[i]  
    fn nums_value_min() { 
        let input = vec![1, 2, -1e5 as i32 - 1, 3, 4];
        let _output = Solution::three_sum(input);
    }

    #[test]
    #[should_panic]
    /// nums[i] <= 1e5
    fn nums_value_max() { 
        let input = vec![1, 2, 1e5 as i32 + 1, 3, 4];
        let _output = Solution::three_sum(input);
    }
}
