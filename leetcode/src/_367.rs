struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut sum: i64 = 0;
        for x in (1..=num as i64).filter(|x| x % 2 == 1) {
            sum += x;
            if sum >= num as i64 {
                return sum == num as i64;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_367() {
        assert_eq!(Solution::is_perfect_square(4), true);
        assert_eq!(Solution::is_perfect_square(1), true);
        assert_eq!(Solution::is_perfect_square(5), false);
        assert_eq!(Solution::is_perfect_square(16), true);
        assert_eq!(Solution::is_perfect_square(2147483647), false);
    }
}
