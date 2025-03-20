struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        while let Some(mut node) = cur.next.take() {
            if node.val == val {
                cur.next = node.next.take();
                continue;
            }
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
        dummy.next.take()
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};
    #[test]
    fn test_203() {
        assert_eq!(
            Solution::remove_elements(Some(Box::new(ListNode::new(7))), 7),
            None
        );
        assert_eq!(
            Solution::remove_elements(
                Some(Box::new(ListNode {
                    val: 7,
                    next: Some(Box::new(ListNode {
                        val: 7,
                        next: Some(Box::new(ListNode {
                            val: 7,
                            next: Some(Box::new(ListNode {
                                val: 7,
                                next: Some(Box::new(ListNode::new(7))),
                            })),
                        })),
                    })),
                })),
                7
            ),
            None
        );
    }
}
