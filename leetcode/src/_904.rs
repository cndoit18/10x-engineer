use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn total_fruit(fruits: &[i32]) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_904() {
        assert_eq!(Solution::total_fruit(&[1, 2]), 2);
        assert_eq!(Solution::total_fruit(&[1, 2, 1]), 3);
        assert_eq!(Solution::total_fruit(&[0, 1, 2, 2]), 3);
        assert_eq!(Solution::total_fruit(&[1, 2, 3, 2, 2]), 4);
        assert_eq!(Solution::total_fruit(&[3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]), 5);
    }
}
