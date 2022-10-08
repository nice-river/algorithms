#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut ans = 1;
        s.into_bytes().windows(2).fold(1, |c, w| {
            if w[0] == w[1] {
                ans = ans.max(c + 1);
                c + 1
            } else {
                1
            }
        });
        ans
    }
}
