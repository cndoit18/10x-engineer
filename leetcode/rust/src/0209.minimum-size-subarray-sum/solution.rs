// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/minimum-size-subarray-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min = None;
        let (mut right, mut left) = (0, 0);

        let mut sum = 0;
        while right < nums.len() {
            sum += nums[right];
            right += 1;

            while sum >= target && left < right {
                if min.is_none() || ((right - left) as i32) < min.unwrap() {
                    min = Some((right - left) as i32)
                }

                sum -= nums[left];
                left += 1;
            }
        }
        min.unwrap_or(0)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let target: i32 = deserialize(&read_line()?)?;
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::min_sub_array_len(target, nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
