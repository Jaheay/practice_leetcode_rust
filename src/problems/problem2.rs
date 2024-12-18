use super::common::listnode::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut result;
        let mut count = 0;

        while l1.is_some() || l2.is_some() {
            assert!(
                count < 100,
                "The depth of the list exceeds the maximum allowed depth of 100."
            );
            let val1 = match &l1 {
                Some(l1) => l1.val,
                None => 0,
            };
            let val2 = match &l2 {
                Some(l2) => l2.val,
                None => 0,
            };

            assert!(val1 < 10 && val2 < 10);
            assert!(val1 >= 0 && val2 >= 0);

            let digit_sum = val1 + val2 + carry;
            carry = digit_sum / 10;

            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(digit_sum % 10)));
            tail = &mut tail.as_mut().unwrap().next;

            l1 = l1.and_then(|mut list| list.next.take());
            l2 = l2.and_then(|mut list| list.next.take());
            count += 1;
        }

        if carry > 0 {
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }

        result.unwrap().next.take()
    }
}

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    /// Example 1:
    /// Input: l1 = [2,4,3], l2 = [5,6,4]
    /// Output: [7,0,8]
    /// Explanation: 342 + 465 = 807.
    fn example1() {
        let input1 = ListNode::from_array([2, 4, 3]);
        let input2 = ListNode::from_array([5, 6, 4]);
        let output = ListNode::to_vec(Solution::add_two_numbers(input1, input2));
        assert_eq!(output, vec![7, 0, 8])
    }

    #[test]
    /// Example 2:
    /// Input: l1 = [0], l2 = [0]
    /// Output: [0]
    fn example2() {
        let input1 = ListNode::from_array([0]);
        let input2 = ListNode::from_array([0]);
        let output = ListNode::to_vec(Solution::add_two_numbers(input1, input2));
        assert_eq!(output, vec![0])
    }

    #[test]
    /// Example 3:
    /// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    /// Output: [8,9,9,9,0,0,0,1]
    fn example3() {
        let input1 = ListNode::from_array([9, 9, 9, 9, 9, 9, 9]);
        let input2 = ListNode::from_array([9, 9, 9, 9]);
        let output = ListNode::to_vec(Solution::add_two_numbers(input1, input2));
        assert_eq!(output, vec![8, 9, 9, 9, 0, 0, 0, 1])
    }
}

#[cfg(test)]
mod constraints {
    use super::*;

    #[test]
    #[should_panic]
    /// The number of nodes in each linked list is in the range `[1, 100]`.
    fn list_len_max() {
        let input1 = ListNode::from_array([3; 101]);
        let input2 = ListNode::from_array([2, 3, 4]);
        let _output = Solution::add_two_numbers(input1, input2);
    }

    #[test]
    #[should_panic]
    /// `0 <= Node.val <= 9`
    fn node_val_min() {
        let input1 = ListNode::from_array([4, -1, 2]);
        let input2 = ListNode::from_array([2, 3, 4]);
        let _output = Solution::add_two_numbers(input1, input2);
    }

    #[test]
    #[should_panic]
    /// `0 <= Node.val <= 9`
    fn node_val_max() {
        let input1 = ListNode::from_array([4, 11, 2]);
        let input2 = ListNode::from_array([2, 3, 4]);
        let _output = Solution::add_two_numbers(input1, input2);
    }
}
