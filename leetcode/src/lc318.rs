#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let words = vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
        let words = words.into_iter().map(|s| s.to_string()).collect();
        let ans = 16;
        assert_eq!(Solution::max_product(words), ans);
    }

    #[test]
    fn test1() {
        let words = vec!["baz", "foo"];
        let words = words.into_iter().map(|s| s.to_string()).collect();
        let ans = 9;
        assert_eq!(Solution::max_product(words), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let words = words
            .into_iter()
            .map(|s| {
                let mut bits = 0;
                for ch in s.as_bytes() {
                    bits |= 1 << (ch - b'a');
                }
                (bits, s.len())
            })
            .collect::<Vec<_>>();
        let mut ans = 0;
        for (i, x) in words.iter().enumerate() {
            for y in &words[i + 1..] {
                if x.0 & y.0 == 0 {
                    ans = ans.max(x.1 * y.1);
                }
            }
        }
        ans as i32
    }
}
