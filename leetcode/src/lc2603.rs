#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let coins = vec![0, 0];
        let edges = vec![vec![0, 1]];
        let ans = 0;
        assert_eq!(Solution::collect_the_coins(coins, edges), ans);
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

use std::collections::HashSet;

impl Solution {
    pub fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();
        let mut tree = vec![HashSet::<usize>::new(); n];
        for edge in edges {
            tree[edge[0] as usize].insert(edge[1] as usize);
            tree[edge[1] as usize].insert(edge[0] as usize);
        }
        let mut from_coin = vec![0; n];
        let mut leaves = vec![];
        for i in 0..n {
            if tree[i].len() == 1 {
                leaves.push(i);
            }
        }
        while let Some(leave) = leaves.pop() {
            if from_coin[leave] == 2 {
                continue;
            }
            if tree[leave].iter().next().is_none() {
                continue;
            }
            let &adj = tree[leave].iter().next().unwrap();
            tree[leave].clear();
            tree[adj].remove(&leave);
            if from_coin[leave] == 1 {
                from_coin[adj] = 2;
            } else if coins[leave] == 1 {
                from_coin[adj] = from_coin[adj].max(1);
            }
            if tree[adj].len() == 1 {
                leaves.push(adj);
            }
        }
        tree.iter().map(|h| h.len()).sum::<usize>() as i32
    }
}
