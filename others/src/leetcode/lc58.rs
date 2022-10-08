#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "  a    b  cd   ".into();
        let ans = 2;
        assert_eq!(Solution::length_of_last_word(s), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut words = s.split(" ");
        let mut ans = 0;
        for w in words {
            if w.len() != 0 {
                ans = w.len() as i32;
            }
        }
        ans
    }
}
