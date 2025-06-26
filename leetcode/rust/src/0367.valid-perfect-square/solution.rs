// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/valid-perfect-square/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut sum: i64 = 0;
        for x in (1..=num as i64).filter(|x| x % 2 == 1) {
            sum += x;
            if sum >= num as i64 {
                return sum == num as i64;
            }
        }
        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_perfect_square(num).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
