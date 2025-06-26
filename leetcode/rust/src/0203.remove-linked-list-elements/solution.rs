// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/remove-linked-list-elements/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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

// @lc code=end

fn main() -> Result<()> {
    let head: LinkedList = deserialize(&read_line()?)?;
    let val: i32 = deserialize(&read_line()?)?;
    let ans: LinkedList = Solution::remove_elements(head.into(), val).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
