struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        let mut prev = None;
        let (mut curr, mut next) = (0, 0);
        while next < nums.len() {
            nums[curr] = nums[next];
            if prev.filter(|&x| nums[curr] == x).is_none() {
                prev = Some(nums[curr]);
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
    fn test_026() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1]), 1);
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }
}
