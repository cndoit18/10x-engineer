// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/move-zeroes/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut curr, mut next) = (0, 0);
        while next < nums.len() {
            nums[curr] = nums[next];
            if nums[curr] != 0 {
                curr += 1;
            }
            next += 1;
        }
        while curr < nums.len() {
            nums[curr] = 0;
            curr += 1;
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut nums: Vec<i32> = deserialize(&read_line()?)?;
    Solution::move_zeroes(&mut nums);
    let ans: Vec<i32> = nums.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
