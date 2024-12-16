use super::common::listnode::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        // Step 1: Calculate the length of the linked list
        let mut list_length = 0;
        // Temporary reference to traverse the list
        let mut length_counter = &head; 
        while let Some(node) = length_counter {
            
            if node.val < 0 || node.val > 9 {
                panic!("Node values must be in the range 0 <= Node.val <= 9");
            }

            length_counter = &node.next;
            list_length += 1;

            if list_length > 1e5 as usize  {
                panic!("The number of nodes in the list exceeds the maximum allowed length of 10^5");
            }
        }

        // Step 2: Traverse to the middle of the list
        let mut middle_pointer = &mut head;
        for _ in 0..(list_length - 1) / 2 {
            middle_pointer = &mut middle_pointer.as_mut().unwrap().next;
        }

        // Step 3: Reverse the second half of the list
        let mut reversed_list = None; 
        // Start at the middle
        let mut second_half = middle_pointer.as_mut().unwrap().next.take(); 
        while let Some(mut current_node) = second_half {
            second_half = current_node.next.take();
            current_node.next = reversed_list;
            reversed_list = Some(current_node);
        }

        // Step 4: Compare the two halves
        // Start of the reversed second half
        let mut right_half = &reversed_list; 
        // Start of the first half
        let mut left_half = &head;          
        while let (Some(right_node), Some(left_node)) = (right_half, left_half) {
            if right_node.val != left_node.val {
                return false; // Mismatch found
            }
            right_half = &right_node.next;
            left_half = &left_node.next;
        }

        true // No mismatches; the list is a palindrome
    }
}



#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: head = [1,2,2,1]
    /// Output: true
    fn example1() {
        let input = ListNode::from_vec(vec![1, 2, 2, 1]);
        let output = Solution::is_palindrome(input);
        assert_eq!(output, true);
    }

    #[test]
    /// Example 2:
    /// Input: head = [1,2]
    /// Output: false
    fn example2() {
        let input = ListNode::from_vec(vec![1, 2]);
        let output = Solution::is_palindrome(input);
        assert_eq!(output, false);
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// `0 <= Node.val <= 9`
    fn node_val_max() {
        let input = ListNode::from_vec(vec![1, 2, 3, 11]);
        let _output = Solution::is_palindrome(input);
    }

    #[test]
    #[should_panic]
    /// `0 <= Node.val <= 9`
    fn node_val_min() {
        let input = ListNode::from_vec(vec![1, 2, 3, -1, 5]);
        let _output = Solution::is_palindrome(input);
    }

    #[test]
    #[should_panic]
    /// The number of nodes in the list is in the range `[1, 1e5]`.
    fn node_len_max() {
        let input = ListNode::from_vec(vec![5; 1e5 as usize + 1]);
        let _output = Solution::is_palindrome(input);
    }
}
