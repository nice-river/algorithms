#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 3;
        let m = 4;
        let k = 3;
        let ans = 4;
        assert_eq!(Solution::num_of_arrays(n, m, k), ans);
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
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MODULE: i64 = 1_000_000_007;
        let n = n as usize;
        let m = m as usize;
        let k = k as usize;
        let mut powers = vec![vec![1; n + 1]; m + 1];
        for i in 1..=m {
            let mut b = i as i64;
            for j in 1..=n {
                powers[i][j] = b;
                b *= i as i64;
                b %= MODULE;
            }
        }
        // let mut dp = vec![vec![0; m + 1]; k + 1];
        let mut sums = vec![vec![0; m + 1]; k + 1];
        let mut ans = 0i64;
        for i in 1..=m {
            sums[1][i] = 1 + sums[1][i - 1];
            if k == 1 {
                ans += powers[i][n - 1];
                ans %= MODULE;
            }
        }
        for i in 1..n {
            let mut ndp = vec![vec![0; m + 1]; k + 1];
            let mut nsums = vec![vec![0; m + 1]; k + 1];
            for s in 1..=k {
                for t in 1..=m {
                    ndp[s][t] += sums[s - 1][t - 1];
                    ndp[s][t] %= MODULE;
                    nsums[s][t] += nsums[s][t - 1] + ndp[s][t];
                    nsums[s][t] %= MODULE;
                }
            }
            sums = nsums;
            for t in 1..=m {
                let mut x = ndp[k][t];
                x *= powers[m][n - i - 1];
                x %= MODULE;
                ans += x;
                ans %= MODULE;
            }
        }
        ans as i32
    }
}
