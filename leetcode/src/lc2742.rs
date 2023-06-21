#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let cost = vec![1, 2, 3, 2];
        let time = vec![1, 2, 3, 2];
        let ans = 3;
        assert_eq!(Solution::paint_walls(cost, time), ans);
    }

    #[test]
    fn test1() {
        let cost = vec![2, 3, 4, 2];
        let time = vec![1, 1, 1, 1];
        let ans = 4;
        assert_eq!(Solution::paint_walls(cost, time), ans);
    }

    #[test]
    fn test2() {
        let cost = vec![42, 8, 28, 35, 21, 13, 21, 35];
        let time = vec![2, 1, 1, 1, 2, 1, 1, 2];
        let ans = 63;
        assert_eq!(Solution::paint_walls(cost, time), ans);
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

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![vec![i32::MAX; n + 1]; n];
        dp[0][0] = 0;
        for i in 1..=(time[0] + 1).min(n as i32) {
            dp[0][i as usize] = cost[0];
        }
        for i in 1..n {
            let mut heap = BinaryHeap::new();
            heap.push((Reverse(0), 0));
            dp[i] = dp[i - 1].clone();
            for j in 1..=n {
                if dp[i - 1][j] != i32::MAX {
                    heap.push((Reverse(dp[i - 1][j]), j));
                }
                let t = (time[i] + 1) as usize;
                while let Some((Reverse(c), p)) = heap.pop() {
                    if p + t >= j {
                        heap.push((Reverse(c), p));
                        dp[i][j] = dp[i][j].min(c + cost[i]);
                        break;
                    }
                }
            }
        }
        dp[n - 1][n]
    }
}
