// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/sqrtx/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut left, mut mid, mut right) = (0, x, x);
        while left < right && right - left > 1 {
            match mid as i64 * mid as i64 {
                sum if sum == x as i64 => return mid,
                sum if sum < x as i64 => {
                    left = mid;
                    mid = ((mid as i64 + right as i64) / 2) as i32;
                }
                _ => {
                    right = mid;
                    mid = ((mid as i64 + left as i64) / 2) as i32;
                }
            }
        }
        mid
    }
}

// @lc code=end

fn main() -> Result<()> {
    let x: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::my_sqrt(x).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
