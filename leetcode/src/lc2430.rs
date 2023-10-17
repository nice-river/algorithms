#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "aaabaab";
        let ans = 4;
        assert_eq!(Solution::delete_string(s.to_owned()), ans);
    }

    #[test]
    fn test1() {
        let s = "tythegdaaaaaaa";
        let ans = 1;
        assert_eq!(Solution::delete_string(s.to_owned()), ans);
    }

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

struct Solution {}

impl Solution {
    pub fn delete_string(s: String) -> i32 {
        const MODULE: i64 = 1700002529; // 1_000_000_007;
        const N: i64 = 26;
        let s = s.as_bytes();
        let mut hashes = vec![0; s.len() + 1];
        let mut pows = vec![1; s.len() + 1];
        for i in 1..=s.len() {
            pows[i] = pows[i - 1] * N % MODULE;
        }
        for i in 0..s.len() {
            let k = (s[i] - b'a') as i64;
            hashes[i + 1] = hashes[i] + k * pows[i] % MODULE;
            hashes[i + 1] %= MODULE;
        }
        let mut dp = vec![1; s.len()];
        for i in (0..s.len() - 1).rev() {
            let mut g = 1;
            for j in i + 1..=(i + s.len()) / 2 {
                let a = (hashes[j] - hashes[i] + MODULE) % MODULE;
                let b = (hashes[j + j - i] - hashes[j] + MODULE) % MODULE;
                if a * pows[j - i] % MODULE == b {
                    g = g.max(1 + dp[j]);
                }
            }
            dp[i] = g;
        }
        dp[0]
    }
}
