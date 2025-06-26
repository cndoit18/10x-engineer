// Created by cndoit18 at 2025/06/26 19:06
// leetgo: 1.4.14
// https://leetcode.cn/problems/fruit-into-baskets/

use std::collections::HashMap;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut select = HashMap::<i32, usize>::new();
        let (mut left, mut right) = (0, 0);
        let mut max = 0;
        while right < fruits.len() {
            select.insert(fruits[right], right);
            right += 1;
            while select.len() > 2 {
                if let Some((&key, &v)) = select.iter().min_by_key(|(_, v)| *v) {
                    select.remove(&key);
                    left = v + 1;
                }
            }
            max = max.max((right - left) as i32);
        }
        max
    }
}

// @lc code=end

fn main() -> Result<()> {
    let fruits: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::total_fruit(fruits).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
