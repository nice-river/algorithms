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

use crate::*;

struct Solution {}

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let n = n as usize;
        let mut roads = vec![vec![]; n];
        for i in 0..n - 1 {
            roads[i].push(i + 1);
        }
        for query in queries {
            let x = query[0] as usize;
            let y = query[1] as usize;
            roads[x].push(y);
            ans.push(Self::calc(&roads));
        }
        ans
    }

    fn calc(roads: &Vec<Vec<usize>>) -> i32 {
        let mut dp = vec![0; roads.len()];
        for i in (0..roads.len() - 1).rev() {
            dp[i] = dp[roads[i][0]] + 1;
            for &nxt in roads[i].iter().skip(1) {
                dp[i] = dp[i].min(dp[nxt] + 1);
            }
        }
        dp[0]
    }
}
