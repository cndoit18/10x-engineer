struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() || (nums.len() == 1 && nums[0] != target) {
            return -1;
        }
        let mid = nums.len() / 2;
        if nums[mid] == target {
            return mid as i32;
        }

        if nums[mid] > target {
            return Solution::search(nums[..mid].to_vec(), target);
        }
        match Solution::search(nums[mid..].to_vec(), target) {
            -1 => -1,
            x => mid as i32 + x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_704() {
        assert_eq!(Solution::search(vec![1], 1), 0);
        assert_eq!(Solution::search(vec![], 1), -1);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 0), 1);
    }
}
