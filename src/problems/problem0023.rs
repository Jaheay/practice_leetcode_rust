use super::common::listnode::ListNode;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
pub struct Solution;

type ListNodeLink = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<ListNodeLink>) -> ListNodeLink {
        // Validate constraints
        let total_node_count: usize = lists
            .iter()
            .enumerate()
            .map(|(list_index, list)| Self::validate_constraints(list, list_index))
            .sum();

        if total_node_count > 10_000 {
            panic!("The total sum of nodes in all lists exceeds the allowed maximum of 10,000.");
        }

        // Initialize the min-heap
        let mut min_heap = BinaryHeap::new();
        for list_head in lists.into_iter().flatten() {
            min_heap.push(list_head);
        }

        // Create a dummy node to simplify the merging process
        let mut prehead_node = Box::new(ListNode::new(0));
        let mut current_merge_position = &mut prehead_node;

        // Merge process
        while let Some(mut smallest_node) = min_heap.pop() {
            if let Some(next_node) = smallest_node.next.take() {
                min_heap.push(next_node);
            }

            current_merge_position.next = Some(smallest_node);
            current_merge_position = current_merge_position.next.as_mut().unwrap();
        }

        prehead_node.next.take()
    }

    /// Validates the constraints for the input list and returns its length
    fn validate_constraints(list: &ListNodeLink, list_index: usize) -> usize {
        let mut current_node = list;
        let mut node_count = 0;
        let mut previous_node_value = None;

        while let Some(node) = current_node {
            node_count += 1;

            if node_count > 500 {
                panic!(
                    "List {} exceeds the allowed maximum length of 500 nodes.",
                    list_index
                );
            }

            if node.val < -10_000 || node.val > 10_000 {
                panic!(
                    "Node value {} in list {} is outside the allowed range [-10,000, 10,000].",
                    node.val, list_index
                );
            }

            if let Some(previous_value) = previous_node_value {
                if previous_value > node.val {
                    panic!(
                        "List {} is not sorted in non-decreasing order at value {}.",
                        list_index, node.val
                    );
                }
            }

            previous_node_value = Some(node.val);
            current_node = &node.next;
        }

        node_count
    }
}

#[cfg(test)]
mod examples {

    use super::*;

    #[test]
    /// Example 1:
    /// Input: list1 = [1,2,4], list2 = [1,3,4]
    /// Output: [1,1,2,3,4,4]
    fn example1() {
        let input_lists = vec![
            ListNode::from_array([1, 4, 5]),
            ListNode::from_array([1, 3, 4]),
            ListNode::from_array([2, 6]),
        ];
        let output = Solution::merge_k_lists(input_lists);
        let example_output = ListNode::from_array([1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(output, example_output);
    }

    #[test]
    /// Example 2:
    /// Input: list1 = [], list2 = []
    /// Output: []
    fn example2() {
        let input_lists = vec![];
        let output = Solution::merge_k_lists(input_lists);
        let example_output = ListNode::from_array([]);
        assert_eq!(output, example_output);
    }

    #[test]
    /// Example 3:
    /// Input: list1 = [], list2 = [0]
    /// Output: [0]
    fn example3() {
        let input_lists = vec![ListNode::from_array([])];
        let output = Solution::merge_k_lists(input_lists);
        let example_output = ListNode::from_array([]);
        assert_eq!(output, example_output);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// 0 <= lists.length <= 1e4
    fn lists_len_max() {
        let input_lists = vec![ListNode::from_array([1; 10]); 1e4 as usize + 1];
        let _output = Solution::merge_k_lists(input_lists);
    }

    #[test]
    #[should_panic]
    /// 0 <= lists[i].length <= 500
    fn node_count_max() {
        let input_lists = vec![ListNode::from_array([1; 501]); 1];
        let _output = Solution::merge_k_lists(input_lists);
    }

    #[test]
    #[should_panic]
    /// -1e4 <= lists[i][j]
    fn node_val_min() {
        let input_lists = vec![ListNode::from_array([-1e4 as i32 - 1]); 1];
        let _output = Solution::merge_k_lists(input_lists);
    }

    #[test]
    #[should_panic]
    /// lists[i][j] <= 1e4
    fn node_val_max() {
        let input_lists = vec![ListNode::from_array([1e4 as i32 + 1]); 1];
        let _output = Solution::merge_k_lists(input_lists);
    }

    #[test]
    #[should_panic]
    /// lists[i] is sorted in ascending order.
    fn node_non_decreasing_sort() {
        let input_lists = vec![ListNode::from_array([5, 4, 3, 2, 1]); 1];
        let _output = Solution::merge_k_lists(input_lists);
    }

    #[test]
    #[should_panic]
    /// The sum of lists[i].length will not exceed 1e4.
    fn sum_of_lists() {
        // Create 21 lists, each with 500 nodes, so the total number of nodes is 21 * 500 = 10,500.
        let input_lists = vec![ListNode::from_array([1; 500]); 21];
        let _output = Solution::merge_k_lists(input_lists);
    }
}
