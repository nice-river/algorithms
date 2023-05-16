#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let s = s.as_bytes();
        if s.len() == 0 {
            return 0;
        }
        let mut zeros = vec![0; s.len()];
        let mut ones = vec![0; s.len()];
        for (i, &c) in s.iter().enumerate() {
            if c == b'0' {
                zeros[i] = if i == 0 { 1 } else { zeros[i - 1] + 1 };
            }
        }
        for (i, &c) in s.iter().enumerate().rev() {
            if c == b'1' {
                ones[i] = if i == s.len() - 1 { 1 } else { ones[i + 1] + 1 };
            }
        }
        let mut ans = 0;
        for i in 0..s.len() - 1 {
            ans = ans.max(zeros[i].min(ones[i + 1]));
        }
        ans * 2
    }
}
