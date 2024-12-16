/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    // Convert from Vec<i32> to ListNode
    // Only used for tests since LeetCode defines ListNode
     pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut iter = vec.into_iter().rev();
        let mut head = None;
        while let Some(val) = iter.next() {
            let node = Box::new(ListNode { val, next: head });
            head = Some(node);
        }
        head
    }

    // Convert from ListNode to Vec<i32>
    // Only used for tests since LeetCode defines ListNode
    pub fn to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(mut node) = list {
            vec.push(node.val);
            list = node.next.take();
        }
        vec
    }

    // Convert from i32 to ListNode
    // Only used for tests since LeetCode defines ListNode
    pub fn from_i32(num: i32) -> Option<Box<ListNode>> {
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
