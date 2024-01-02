#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let source = "abcd".to_owned();
        let target = "acbe".to_owned();
        let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
        let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
        let cost = vec![2, 5, 5, 1, 2, 20];
        let ans = 28;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            ans
        );
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

use crate::*;

struct Solution {}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        const N: usize = 26;
        const BLOCK: i64 = i64::MAX / 8;
        let source = source.into_bytes();
        let target = target.into_bytes();
        let mut dist = vec![vec![BLOCK; N]; N];
        for i in 0..N {
            dist[i][i] = 0;
        }
        for i in 0..original.len() {
            let a = (original[i] as u8 - b'a') as usize;
            let b = (changed[i] as u8 - b'a') as usize;
            dist[a][b] = dist[a][b].min(cost[i] as i64);
        }
        for k in 0..N {
            for i in 0..N {
                for j in 0..N {
                    if dist[i][j] > dist[i][k] + dist[k][j] {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }
        let mut ans = 0;
        for (a, b) in source.into_iter().zip(target.into_iter()) {
            let a = (a - b'a') as usize;
            let b = (b - b'a') as usize;
            if dist[a][b] != BLOCK {
                ans += dist[a][b];
            } else {
                return -1;
            }
        }
        ans
    }
}
