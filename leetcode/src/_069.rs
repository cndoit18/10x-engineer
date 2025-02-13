struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut left, mut mid, mut right) = (0, x, x);
        while left < right && right - left > 1 {
            match mid as i64 * mid as i64 {
                sum if sum == x as i64 => return mid,
                sum if sum < x as i64 => {
                    left = mid;
                    mid = ((mid as i64 + right as i64) / 2) as i32;
                }
                _ => {
                    right = mid;
                    mid = ((mid as i64 + left as i64) / 2) as i32;
                }
            }
        }
        mid
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_069() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(5), 2);
        assert_eq!(Solution::my_sqrt(6), 2);
        assert_eq!(Solution::my_sqrt(7), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
        assert_eq!(Solution::my_sqrt(10), 3);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
