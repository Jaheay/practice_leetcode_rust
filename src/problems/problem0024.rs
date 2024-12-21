use super::common::listnode::ListNode;

type ListNodeLink = Option<Box<ListNode>>;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: ListNodeLink) -> ListNodeLink {
        // Validate the constraints
        Solution::validate_constraints(&head);

        let mut current_node = &mut head;

        while let Some(node) = current_node.as_mut() {
            if let Some(mut next_node) = node.next.take() {
                // Perform the swap
                node.next = next_node.next.take();
                next_node.next = Some(node.clone());
                *current_node = Some(next_node);
                current_node = &mut current_node.as_mut().unwrap().next.as_mut().unwrap().next;
            } else {
                break;
            }
        }

        head
    }

    fn validate_constraints(head: &ListNodeLink) {
        let mut node_count = 0;
        let mut current_node = head;

        while let Some(node) = current_node {
            node_count += 1;

            if node_count > 100 {
                panic!("The number of nodes in the list exceeds the allowed maximum of 100.");
            }

            if node.val < 0 {
                panic!("Node value {} is below the allowed minimum of 0.", node.val);
            }

            if node.val > 100 {
                panic!(
                    "Node value {} exceeds the allowed maximum of 100.",
                    node.val
                );
            }

            current_node = &node.next;
        }
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: head = [1,2,3,4]
    /// Output: [2,1,4,3]
    fn example1() {
        let input_list = ListNode::from_array([1, 2, 3, 4]);
        let expected_output = ListNode::from_array([2, 1, 4, 3]);
        assert_eq!(Solution::swap_pairs(input_list), expected_output);
    }

    #[test]
    /// Example 2:
    /// Input: head = []
    /// Output: []
    fn example2() {
        let input_list = ListNode::from_array([]);
        let expected_output = ListNode::from_array([]);
        assert_eq!(Solution::swap_pairs(input_list), expected_output);
    }

    #[test]
    /// Example 3:
    /// Input: head = [1]
    /// Output: [1]
    fn example3() {
        let input_list = ListNode::from_array([1]);
        let expected_output = ListNode::from_array([1]);
        assert_eq!(Solution::swap_pairs(input_list), expected_output);
    }

    #[test]
    /// Example 4:
    /// Input: head = [1,2,3]
    /// Output: [2,1,3]
    fn example4() {
        let input_list = ListNode::from_array([1, 2, 3]);
        let expected_output = ListNode::from_array([2, 1, 3]);
        assert_eq!(Solution::swap_pairs(input_list), expected_output);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// The number of nodes in the list is in the range [0, 100].
    fn node_count_max() {
        // Create a list with 101 nodes
        let input_list = ListNode::from_array([0; 101]);
        let _output = Solution::swap_pairs(input_list);
    }

    #[test]
    #[should_panic]
    /// 0 <= Node.val
    fn node_val_min() {
        // Create a list with a node value below 0
        let input_list = ListNode::from_array([-1]);
        let _output = Solution::swap_pairs(input_list);
    }

    #[test]
    #[should_panic]
    /// Node.val <= 100
    fn node_val_max() {
        // Create a list with a node value above 100
        let input_list = ListNode::from_array([101]);
        let _output = Solution::swap_pairs(input_list);
    }
}
