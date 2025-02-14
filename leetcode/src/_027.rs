struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        let (mut curr, mut next) = (0, 0);
        while next < nums.len() {
            nums[curr] = nums[next];
            if nums[curr] != val {
                curr += 1;
            }
            next += 1;
        }
        curr as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_027() {
        assert_eq!(Solution::remove_element(&mut vec![1], 1), 0);
        assert_eq!(Solution::remove_element(&mut vec![3, 2, 2, 3], 3), 2);
        assert_eq!(
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
            5
        );
    }
}
