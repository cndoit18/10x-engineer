// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/search-insert-position/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return if nums[0] >= target { 0 } else { 1 };
        }

        let mid = nums.len() / 2;
        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] > target {
            return Solution::search_insert(nums[..mid].to_vec(), target);
        }
        mid as i32 + Solution::search_insert(nums[mid..].to_vec(), target)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::search_insert(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
