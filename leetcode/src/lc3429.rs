#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let cost = to_vec!([[3,5,7],[6,2,9],[4,8,1],[7,3,5]]);
        let ans = 9;
        assert_eq!(Solution::min_cost(n, cost), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        const PAIRS: [(usize, usize); 6] = [
            (0, 1), (1, 0), (0, 2), (2, 0), (1, 2), (2, 1),
        ];
        let mut dp = vec![0; 6];
        for (i, &p) in PAIRS.iter().enumerate() {
            dp[i] = cost[0][p.0]  as i64 + cost[n - 1][p.1] as i64;
        }
        for i in 1..n / 2{
            let mut nxt = vec![i64::MAX; 6];
            for j in 0..6 {
                for k in 0..6 {
                    if PAIRS[k].0 != PAIRS[j].0 && PAIRS[k].1 != PAIRS[j].1 {
                        let v = cost[i][PAIRS[k].0] as i64 + cost[n - 1 - i][PAIRS[k].1] as i64;
                        nxt[k] = nxt[k].min(dp[j] + v);
                    }
                }
            }
            dp = nxt;
        }
        dp.into_iter().min().unwrap()
    }
}