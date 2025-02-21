struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: &[i32]) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_209() {
        assert_eq!(Solution::min_sub_array_len(4, &[1, 4, 4]), 1);
        assert_eq!(Solution::min_sub_array_len(7, &[2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(
            Solution::min_sub_array_len(11, &[1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
        assert_eq!(Solution::min_sub_array_len(11, &[1, 2, 3, 4, 5]), 3);
    }
}
