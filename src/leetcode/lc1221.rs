#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut ans = 0;
        let mut r = 0;
        let mut l = 0;
        for &ch in s.as_bytes() {
            match ch {
                b'R' => r += 1,
                b'L' => l += 1,
                _ => unreachable!(),
            }
            if r == l {
                ans += 1;
            }
        }
        ans
    }
}
