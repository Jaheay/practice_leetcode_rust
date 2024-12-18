use std::cmp::Ordering;

type ListNodeLink = Option<Box<ListNode>>;
/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListNodeLink,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    /// Convert from Vec<i32> to ListNode
    /// Only used for tests since LeetCode defines ListNode
    pub fn from_vec(vec: Vec<i32>) -> ListNodeLink {
        let mut iter = vec.into_iter().rev();
        let mut head = None;
        for val in iter {
            let node = Box::new(ListNode { val, next: head });
            head = Some(node);
        }
        head
    }

    /// Convert from ListNode to Vec<i32>
    /// Only used for tests since LeetCode defines ListNode
    pub fn to_vec(mut list: ListNodeLink) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(mut node) = list {
            vec.push(node.val);
            list = node.next.take();
        }
        vec
    }

    /// Convert from [i32; N] to ListNode
    /// Only used for tests since LeetCode defines ListNode
    pub fn from_array<const N: usize>(arr: [i32; N]) -> ListNodeLink {
        arr.into_iter()
            .rev()
            .fold(None, |next, val| Some(Box::new(ListNode { val, next })))
    }

    /// Convert from ListNode to [i32; N]
    /// Only used for tests since LeetCode defines ListNode
    pub fn to_array<const N: usize>(list: ListNodeLink) -> [i32; N] {
        let mut vec = Vec::with_capacity(N);
        let mut current = list;

        while let Some(mut node) = current {
            vec.push(node.val);
            current = node.next.take();
        }

        let arr: [i32; N] = vec
            .try_into()
            .unwrap_or_else(|_| panic!("ListNode size does not match the array size {}", N));
        arr
    }

    /// Convert from i32 to ListNode
    /// Only used for tests since LeetCode defines ListNode
    pub fn from_i32(num: i32) -> ListNodeLink {
        let mut number = num;
        let mut head = None;

        while number > 0 {
            let node = Box::new(ListNode {
                val: number % 10,
                next: head,
            });
            head = Some(node);
            number /= 10;
        }

        head
    }
}

// Prevents stack overflow on Problem 234
// The default Drop implementation for ListNode is going to be recursive.
// This causes a stack overflow when dropping a list with many items
// Only used for tests since LeetCode defines ListNode
impl Drop for ListNode {
    fn drop(&mut self) {
        let mut node = self.next.take();

        while let Some(mut i) = node {
            node = i.next.take();
        }
    }
}

/// Implement PartialOrd and Ord for ListNode
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self)) // Reverse ordering for min-heap
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}
