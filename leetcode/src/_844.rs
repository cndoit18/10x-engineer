struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut x, mut y) = (Vec::new(), Vec::new());
        for c in s.chars() {
            if c == '#' {
                x.pop();
                continue;
            }
            x.push(c)
        }
        for c in t.chars() {
            if c == '#' {
                y.pop();
                continue;
            }
            y.push(c)
        }
        x.eq(&y)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_844() {
        assert_eq!(
            Solution::backspace_compare("test".to_string(), "test".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a#c".to_string(), "b".to_string()),
            false
        );
    }
}
