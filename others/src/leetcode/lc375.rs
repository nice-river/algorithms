#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceChooseIter;

    #[test]
    fn test0() {
        let n = 20;
        let ans = 49;
        assert_eq!(Solution::get_money_amount(n), ans);
    }

    #[test]
    fn test1() {
        let n = 3;
        let ans = 2;
        assert_eq!(Solution::get_money_amount(n), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in (1..n).rev() {
            for j in i..=n {
                let mut v = i32::MAX;
                for k in i..j {
                    let a = dp[i][k - 1];
                    let b = dp[k + 1][j];
                    v = v.min(k as i32 + a.max(b));
                    dp[i][j] = v;
                }
            }
        }
        dp[1][n]
    }
}
