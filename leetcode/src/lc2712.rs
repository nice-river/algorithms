#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "0011".to_owned();
        let ans = 2;
        assert_eq!(Solution::minimum_cost(s), ans);
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
    pub fn minimum_cost(s: String) -> i64 {
        let s = s.into_bytes();
        let s: Vec<usize> = s.into_iter().map(|x| (x - b'0') as usize).collect();
        let n = s.len();
        if n == 1 {
            return 0;
        }
        let mut ans = i64::MAX;
        let mut f = vec![vec![0; 2]; n];
        let mut b = vec![vec![0; 2]; n];
        f[0][0] = (s[0] ^ 0) as i64;
        f[0][1] = (s[0] ^ 1) as i64;
        for i in 1..n {
            if s[i] == 0 {
                f[i][0] = f[i - 1][0];
                f[i][1] = (i + 1) as i64 + f[i - 1][0];
            } else {
                f[i][0] = (i + 1) as i64 + f[i - 1][1];
                f[i][1] = f[i - 1][1];
            }
        }
        b[n - 1][0] = (s[n - 1] ^ 0) as i64;
        b[n - 1][1] = (s[n - 1] ^ 1) as i64;
        for i in (0..n - 1).rev() {
            if s[i] == 0 {
                b[i][0] = b[i + 1][0];
                b[i][1] = (n - i) as i64 + b[i + 1][0];
            } else {
                b[i][0] = (n - i) as i64 + b[i + 1][1];
                b[i][1] = b[i + 1][1];
            }
        }
        for i in 0..n - 1 {
            ans = ans.min(f[i][0] + b[i + 1][0]);
            ans = ans.min(f[i][1] + b[i + 1][1]);
        }
        ans
    }
}
