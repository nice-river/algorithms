#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let k = 2;
        let ans = 9;
        assert_eq!(Solution::nth_smallest(n, k), ans);
    }

    #[test]
    fn test1() {
        let n = 3;
        let k = 1;
        let ans = 4;
        assert_eq!(Solution::nth_smallest(n, k), ans);
    }

    #[test]
    fn test2() {
        let n = 924;
        let k = 6;
        let ans = 4032;
        assert_eq!(Solution::nth_smallest(n, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn nth_smallest(mut n: i64, k: i32) -> i64 {
        let mut k = k as usize;
        let max_zero_cnt = 60;
        let mut dp = vec![vec![0i64; max_zero_cnt]; k + 1];
        for i in 0..max_zero_cnt {
            dp[0][i] = 1;
        }
        for i in 0..k + 1 {
            dp[i][0] = 1;
        }
        for i in 1..k + 1 {
            for j in 1..max_zero_cnt {
                if dp[i - 1][j] == i64::MAX || dp[i][j - 1] == i64::MAX {
                    dp[i][j] = i64::MAX;
                    break;
                }
                if let Some(x) = dp[i - 1][j].checked_add(dp[i][j - 1]) {
                    dp[i][j] = x;
                } else {
                    dp[i][j] = i64::MAX;
                    break;
                }
            }
        }
        let mut ans_bits = vec![];
        ans_bits.push(1);
        k -= 1;
        for i in 0..max_zero_cnt {
            if dp[k][i] == n {
                ans_bits.extend([1].repeat(k));
                ans_bits.extend([0].repeat(i));
                break;
            } else if dp[k][i] > n {
                let mut zero_cnt = i;
                loop {
                    if zero_cnt > 0 && dp[k][zero_cnt - 1] >= n {
                        ans_bits.push(0);
                        zero_cnt -= 1;
                    } else if k > 0 {
                        ans_bits.push(1);
                        if zero_cnt > 0 {
                            n -= dp[k][zero_cnt - 1];
                        }
                        k -= 1;
                    } else {
                        break;
                    }
                }
                break;
            } else {
                n -= dp[k][i];
            }
        }
        let mut ans = 0;
        for b in ans_bits {
            ans <<= 1;
            ans += b;
        }
        ans
    }
}
