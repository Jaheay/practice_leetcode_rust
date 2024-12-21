use super::common::listnode::ListNode;
use std::cmp::Ordering;

pub struct Solution;

type ListNodeLink = Option<Box<ListNode>>;
impl Solution {
    pub fn merge_two_lists(mut list1: ListNodeLink, mut list2: ListNodeLink) -> ListNodeLink {
        let mut prehead = ListNode::new(-1);
        let mut current_node = &mut prehead;

        // Validate constraints
        Solution::validate_constraints(&list1);
        Solution::validate_constraints(&list2);

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            match node1.val.cmp(&node2.val) {
                Ordering::Less => {
                    current_node.next = list1.take();
                    current_node = current_node.next.as_mut().unwrap();
                    list1 = current_node.next.take();
                }
                _ => {
                    current_node.next = list2.take();
                    current_node = current_node.next.as_mut().unwrap();
                    list2 = current_node.next.take();
                }
            }
        }

        current_node.next = list1.or(list2);
        prehead.next.take()
    }

    /// Validates the constraints for the input list
    fn validate_constraints(list: &ListNodeLink) {
        let mut current = list;
        let mut count = 0;
        let mut prev_val = None;

        while let Some(node) = current {
            count += 1;

            // Check node count constraint
            if count > 50 {
                panic!("The number of nodes in the list exceeds 50.");
            }

            // Check value constraints
            if node.val < -100 || node.val > 100 {
                panic!(
                    "Node value {} is out of the allowed range [-100, 100].",
                    node.val
                );
            }

            // Check sorted order
            if let Some(prev) = prev_val {
                if prev > node.val {
                    panic!("The list is not sorted in non-decreasing order.");
                }
            }

            prev_val = Some(node.val);
            current = &node.next;
        }
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
        let input_list1 = ListNode::from_array([1, 2, 4]);
        let input_list2 = ListNode::from_array([1, 3, 4]);
        let output = Solution::merge_two_lists(input_list1, input_list2);
        let example_output = ListNode::from_array([1, 1, 2, 3, 4, 4]);
        assert_eq!(output, example_output);
    }

    #[test]
    /// Example 2:
    /// Input: list1 = [], list2 = []
    /// Output: []
    fn example2() {
        let input_list1 = ListNode::from_array([]);
        let input_list2 = ListNode::from_array([]);
        let output = Solution::merge_two_lists(input_list1, input_list2);
        let example_output = ListNode::from_array([]);
        assert_eq!(output, example_output)
    }

    #[test]
    /// Example 3:
    /// Input: list1 = [], list2 = [0]
    /// Output: [0]
    fn example3() {
        let input_list1 = ListNode::from_array([]);
        let input_list2 = ListNode::from_array([0]);
        let output = Solution::merge_two_lists(input_list1, input_list2);
        let example_output = ListNode::from_array([0]);
        assert_eq!(output, example_output)
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// The number of nodes in both lists is in the range [0, 50].
    fn node_count_max() {
        let input_list1 = ListNode::from_array([1; 51]);
        let input_list2 = ListNode::from_array([0]);
        let _output = Solution::merge_two_lists(input_list1, input_list2);
    }

    #[test]
    #[should_panic]
    /// -100 <= Node.val <= 100
    fn node_val_min() {
        let input_list1 = ListNode::from_array([-101, 1, 2, 3, 4]);
        let input_list2 = ListNode::from_array([0]);
        let _output = Solution::merge_two_lists(input_list1, input_list2);
    }

    #[test]
    #[should_panic]
    /// -100 <= Node.val <= 100
    fn node_val_max() {
        let input_list1 = ListNode::from_array([0]);
        let input_list2 = ListNode::from_array([1, 2, 3, 4, 101]);
        let _output = Solution::merge_two_lists(input_list1, input_list2);
    }

    #[test]
    #[should_panic]
    /// Both list1 and list2 are sorted in non-decreasing order.
    fn node_non_decreasing_sort() {
        let input_list1 = ListNode::from_array([5, 4, 3, 2, 1]);
        let input_list2 = ListNode::from_array([-5, -6, -7, -8, -9]);
        let _output = Solution::merge_two_lists(input_list1, input_list2);
    }
}
