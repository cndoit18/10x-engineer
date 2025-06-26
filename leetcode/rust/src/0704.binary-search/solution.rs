// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/binary-search/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() || (nums.len() == 1 && nums[0] != target) {
            return -1;
        }
        let mid = nums.len() / 2;
        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] > target {
            return Solution::search(nums[..mid].to_vec(), target);
        }
        match Solution::search(nums[mid..].to_vec(), target) {
            -1 => -1,
            x => mid as i32 + x,
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::search(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
