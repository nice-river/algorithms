#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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
    pub fn min_insertions(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let t = s.clone().into_iter().rev().collect::<Vec<_>>();
        let mut dp = vec![vec![0; s.len() + 1]; s.len() + 1];
        for i in 1..=s.len() {
            for j in 1..=s.len() {
                dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
                }
            }
        }
        (s.len() - dp[s.len()][t.len()]) as i32
    }
}
