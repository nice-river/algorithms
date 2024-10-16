#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 2;
        let x = 3;
        let y = 3;
        let ans = 63;
        assert_eq!(Solution::number_of_ways(n, x, y), ans);
        let n = 2;
        let x = 4;
        let y = 2;
        let ans = 56;
        assert_eq!(Solution::number_of_ways(n, x, y), ans);
        let n = 3;
        let x = 2;
        let y = 3;
        let ans = 60;
        assert_eq!(Solution::number_of_ways(n, x, y), ans);
        let n = 5;
        let x = 2;
        let y = 1;
        let ans = 32;
        assert_eq!(Solution::number_of_ways(n, x, y), ans);
    }

    #[test]
    fn test1() {
        let n = 3;
        let x = 3;
        let y = 3;
        let ans = 333;
        assert_eq!(Solution::number_of_ways(n, x, y), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
        const MODULE: i64 = 1e9 as i64 + 7;
        let mut ans = 0i64;
        let mut y_pow = y as i64;
        let mut x_fac = x as i64;
        let mut dp = vec![vec![0; n as usize + 1]; 2];
        let mut cur = 0;
        dp[0][0] = 1;
        for i in 1..=(n.min(x)) as usize {
            let nxt = cur ^ 1;
            for j in i..=n as usize {
                dp[nxt][j] = (dp[nxt][j - 1] * (i as i64) % MODULE + dp[cur][j - 1]) % MODULE;
            }
            ans += y_pow * x_fac % MODULE * dp[nxt][n as usize] % MODULE;
            ans %= MODULE;
            y_pow = y_pow * y as i64 % MODULE;
            x_fac = x_fac * (x as i64 - i as i64) % MODULE;
            dp[cur].fill(0);
            cur ^= 1;
        }
        ans as i32
    }
}
