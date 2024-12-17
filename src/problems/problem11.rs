pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut current_water_level = 0;
        let mut current_area = 0;

        // Verify length contraints
        if height.len() < 2 {
            panic!("The number of provided heights is less than 2")
        }
        if height.len() > 1e5 as usize {
            panic!("The number of provided heights is greater than 100,000")
        }

        // Validate values without consuming the iterator
        if !height.iter().all(|&h| h >= 0 && h <= 1e4 as i32) {
            panic!("All height values must be in the range 0 < height < 10^4");
        }

        // Convert height from container to enumerated iterator
        let mut height = height.iter().enumerate();
        // https://doc.rust-lang.org/nightly/std/iter/trait.DoubleEndedIterator.html
        let mut front = height.next(); // Start from the front
        let mut back = height.next_back(); // Start from the back

        while let (Some((front_index, &front_height)), Some((back_index, &back_height))) =
            (front, back)
        {
            let current_water_level = std::cmp::min(front_height, back_height);
            let current_width = (back_index - front_index) as i32;
            let current_area = current_water_level * current_width;
            max_area = max_area.max(current_area);

            // Whichever height is shorter, iterate that one
            if front_height < back_height {
                front = height.next();
            } else {
                back = height.next_back()
            }

            if front_index >= back_index {
                break;
            }
        }

        return max_area;
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: height = [1,8,6,2,5,4,8,3,7]
    /// Output: 49
    fn example1() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output = Solution::max_area(input);
        assert_eq!(output, 49)
    }

    #[test]
    /// Example 2:
    /// Input: height = [1,1]
    /// Output: 1
    fn example2() {
        let input = vec![1, 1];
        let output = Solution::max_area(input);
        assert_eq!(output, 1);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// n == height.length
    /// 2 <= n <= 10e5
    fn container_len_min() {
        let input = vec![1];
        let _output = Solution::max_area(input);
    }

    #[test]
    #[should_panic]
    /// n == height.length
    /// 2 <= n <= 10e5
    fn container_len_max() {
        let input = vec![5; 1e5 as usize + 1];
        let _output = Solution::max_area(input);
    }

    #[test]
    #[should_panic]
    /// 0 <= height[i] <= 10e4
    fn height_min() {
        let input = vec![1, 2, -1, 3, 4];
        let _output = Solution::max_area(input);
    }

    #[test]
    #[should_panic]
    /// 0 <= height[i] <= 10e4
    fn height_max() {
        let input = vec![1, 2, 1e4 as i32 + 1, 3, 4];
        let _output = Solution::max_area(input);
    }
}
