// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/remove-element/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut curr, mut next) = (0, 0);
        while next < nums.len() {
            nums[curr] = nums[next];
            if nums[curr] != val {
                curr += 1;
            }
            next += 1;
        }
        curr as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    let val: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::remove_element(&mut nums, val).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
