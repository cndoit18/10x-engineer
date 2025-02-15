struct Solution {}

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

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_977() {
        assert_eq!(Solution::sorted_squares(vec![1]), vec![1]);
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
