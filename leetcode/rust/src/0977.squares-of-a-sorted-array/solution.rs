// Created by cndoit18 at 2025/06/26 19:07
// leetgo: 1.4.14
// https://leetcode.cn/problems/squares-of-a-sorted-array/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut right, mut left, mut index) = (0i32, nums.len() as i32 - 1, nums.len() as i32 - 1);
        let mut result = vec![0; index as usize + 1];
        while left >= right {
            if left < 0 || nums[right as usize].abs() > nums[left as usize].abs() {
                result[index as usize] = nums[right as usize] * nums[right as usize];
                right += 1;
            } else {
                result[index as usize] = nums[left as usize] * nums[left as usize];
                left -= 1;
            }
            index -= 1;
        }
        result
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::sorted_squares(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
