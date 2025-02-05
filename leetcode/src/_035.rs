struct Solution {}

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

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_035() {
        assert_eq!(Solution::search_insert(vec![1], 1), 0);
        assert_eq!(Solution::search_insert(vec![], 1), 0);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
