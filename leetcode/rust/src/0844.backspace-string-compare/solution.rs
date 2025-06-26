// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/backspace-string-compare/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut x, mut y) = (Vec::new(), Vec::new());
        for c in s.chars() {
            if c == '#' {
                x.pop();
                continue;
            }
            x.push(c)
        }
        for c in t.chars() {
            if c == '#' {
                y.pop();
                continue;
            }
            y.push(c)
        }
        x.eq(&y)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::backspace_compare(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
