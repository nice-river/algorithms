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
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut ans = vec![];
        for query in queries {
            ans.push(Self::is_match(query, pattern.clone()));
        }
        ans
    }

    fn is_match(a: String, b: String) -> bool {
        let a = a.chars().collect::<Vec<_>>();
        let b = b.chars().collect::<Vec<_>>();
        if !a
            .iter()
            .filter(|c| c.is_uppercase())
            .eq(b.iter().filter(|c| c.is_uppercase()))
        {
            return false;
        }
        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
        for i in 0..a.len() {
            for j in 0..b.len() {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
                if a[i] == b[j] {
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
                }
            }
        }
        dp[a.len()][b.len()] == b.len()
    }
}
