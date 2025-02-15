struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut [i32]) {
        let (mut curr, mut next) = (0, 0);
        while next < nums.len() {
            nums[curr] = nums[next];
            if nums[curr] != 0 {
                curr += 1;
            }
            next += 1;
        }
        while curr < nums.len() {
            nums[curr] = 0;
            curr += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    struct TestCase {
        input: Vec<i32>,
        expect: Vec<i32>,
    }
    #[test]
    fn test_283() {
        for mut case in [TestCase {
            input: vec![0, 1, 0, 3, 12],
            expect: vec![1, 3, 12, 0, 0],
        }] {
            Solution::move_zeroes(&mut case.input);
            assert_eq!(case.input, case.expect);
        }
    }
}
