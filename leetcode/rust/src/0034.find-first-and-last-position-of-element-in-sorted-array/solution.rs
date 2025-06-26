// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() || (nums.len() == 1 && nums[0] != target) {
            return vec![-1, -1];
        }
        let mid = nums.len() / 2;
        if nums[mid] == target {
            return vec![
                (0..mid)
                    .rev()
                    .find(|&n| nums[n] != target)
                    .map(|n| n + 1)
                    .unwrap_or(0) as i32,
                (mid + 1..nums.len())
                    .find(|&n| nums[n] != target)
                    .map(|n| n - 1)
                    .unwrap_or(nums.len() - 1) as i32,
            ];
        }
        if nums[mid] > target {
            return Solution::search_range(nums[..mid].to_vec(), target);
        }

        Solution::search_range(nums[mid..].to_vec(), target)
            .into_iter()
            .map(|x| if x == -1 { -1 } else { x + mid as i32 })
            .collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::search_range(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
