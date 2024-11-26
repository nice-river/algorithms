#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let edges = to_vec!([[0, 1, 3], [1, 2, 5], [2, 3, 4]]);
        let k = 1;
        let ans = 7;
        assert_eq!(Solution::maximize_sum_of_weights(edges, k), ans);
    }

    #[test]
    fn test1() {
        let edges = to_vec!([
            [0, 1, 5],
            [1, 2, 10],
            [0, 3, 15],
            [3, 4, 20],
            [3, 5, 5],
            [0, 6, 10]
        ]);
        let k = 3;
        let ans = 65;
        assert_eq!(Solution::maximize_sum_of_weights(edges, k), ans);
    }

    #[test]
    fn test2() {
        let edges = to_vec!([[0, 1, 4], [0, 2, 2], [2, 3, 12], [2, 4, 6]]);
        let k = 2;
        let ans = 22;
        assert_eq!(Solution::maximize_sum_of_weights(edges, k), ans);
    }

    #[test]
    fn test3() {
        let edges = to_vec!([
            [0, 1, 4],
            [1, 2, 6],
            [1, 3, 2],
            [0, 4, 5],
            [4, 5, 6],
            [4, 6, 5],
            [6, 7, 20],
            [6, 8, 20],
        ]);
        let k = 2;
        let ans = 61;
        assert_eq!(Solution::maximize_sum_of_weights(edges, k), ans);
    }
    #[test]
    fn test4() {
        let edges = to_vec!([[0, 1, 34], [0, 2, 17]]);
        let k = 1;
        let ans = 34;
        assert_eq!(Solution::maximize_sum_of_weights(edges, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2] as i64;
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        let ans = Self::dfs(0, n, &graph, 0, k as usize);
        ans.1
    }

    fn dfs(
        node: usize,
        parent: usize,
        graph: &Vec<Vec<(usize, i64)>>,
        pw: i64,
        k: usize,
    ) -> (i64, i64) {
        let mut ret = vec![];
        for &(child, w) in &graph[node] {
            if child != parent {
                ret.push(Self::dfs(child, node, graph, w, k));
            }
        }
        if graph[node].len() <= k {
            let s = ret.into_iter().map(|(a, b)| a.max(b)).sum();
            (s, s + pw)
        } else {
            ret.sort_by_key(|(a, b)| b - a);
            let k = graph[node].len() - k;
            let del = ret.iter().take(k - 1).map(|(a, b)| a).sum::<i64>()
                + ret.iter().skip(k - 1).map(|(a, b)| a.max(b)).sum::<i64>();
            let not = ret.iter().take(k).map(|(a, b)| a).sum::<i64>()
                + ret.iter().skip(k).map(|(a, b)| a.max(b)).sum::<i64>()
                + pw;
            (del, not)
        }
    }
}
