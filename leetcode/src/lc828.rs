#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "AA".to_owned();
        let ans = 2;
        assert_eq!(Solution::unique_letter_string(s), ans);
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
    pub fn unique_letter_string(s: String) -> i32 {
        let s = s
            .as_bytes()
            .iter()
            .map(|&e| (e - b'A') as usize)
            .collect::<Vec<_>>();
        let n = s.len();
        let mut pos = vec![vec![(0, 0); 30]; n];
        for i in 1..n {
            pos[i] = pos[i - 1].clone();
            let b = pos[i][s[i - 1]].1;
            pos[i][s[i - 1]].0 = b;
            pos[i][s[i - 1]].1 = i;
        }
        let mut dp = vec![0; n];
        dp[0] = 1;
        for i in 1..n {
            let a = pos[i][s[i]].0;
            let b = pos[i][s[i]].1;
            dp[i] = dp[i - 1];
            dp[i] += i + 1 - b;
            dp[i] -= b - a;
        }
        dp.iter().sum::<usize>() as i32
    }
}
