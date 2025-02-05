struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() || (nums.len() == 1 && nums[0] != target) {
            return vec![-1, -1];
        }
        let mid = nums.len() / 2;
        if nums[mid] == target {
            return vec![
                (0..mid)
                    .rev()
                    .find(|&n| nums[n] != target)
                    .map(|n| n + 1)
                    .unwrap_or(0) as i32,
                (mid + 1..nums.len())
                    .find(|&n| nums[n] != target)
                    .map(|n| n - 1)
                    .unwrap_or(nums.len() - 1) as i32,
            ];
        }
        if nums[mid] > target {
            return Solution::search_range(nums[..mid].to_vec(), target);
        }
        Solution::search_range(nums[mid..].to_vec(), target)
            .into_iter()
            .map(|x| if x == -1 { -1 } else { x + mid as i32 })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_034() {
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![2, 2], 2), vec![0, 1]);
        assert_eq!(Solution::search_range(vec![1, 2, 3], 3), vec![2, 2]);
        assert_eq!(Solution::search_range(vec![2, 2], 3), vec![-1, -1]);
    }
}
