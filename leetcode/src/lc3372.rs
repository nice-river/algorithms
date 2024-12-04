#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let edges1 = to_vec!([[0, 1], [0, 2], [0, 3], [0, 4]]);
        let edges2 = to_vec!([[0, 1], [1, 2], [2, 3]]);
        let k = 1;
        let ans = vec![6, 3, 3, 3, 3];
        assert_eq!(Solution::max_target_nodes(edges1, edges2, k), ans);
    }

    #[test]
    fn test1() {
        let edges1 = to_vec!([[0, 1], [0, 2], [2, 3], [2, 4]]);
        let edges2 = to_vec!([[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]]);
        let k = 2;
        let ans = vec![9, 7, 9, 8, 8];
        assert_eq!(Solution::max_target_nodes(edges1, edges2, k), ans);
    }

    #[test]
    fn test2() {
        let edges1 = to_vec!([[3, 0], [3, 1], [2, 3]]);
        let edges2 = to_vec!([
            [4, 0],
            [7, 3],
            [4, 6],
            [9, 4],
            [5, 7],
            [5, 8],
            [2, 5],
            [1, 2],
            [1, 9]
        ]);
        let k = 6;
        let ans = vec![14, 14, 14, 14];
        assert_eq!(Solution::max_target_nodes(edges1, edges2, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let dist1 = Self::build(edges1);
        let dist2 = Self::build(edges2);
        let mut ans = Vec::with_capacity(dist1.len());
        let mut t = 0;
        if k > 0 {
            let x = (k as usize - 1).min(dist2.len() - 1);
            for v in &dist2 {
                t = t.max(v[x]);
            }
        }
        (0..dist1.len()).for_each(|i| {
            ans.push(dist1[i][(k as usize).min(dist1[i].len() - 1)] + t);
        });
        ans
    }

    fn build(edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = edges.len() + 1;
        let mut tree = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].push(v);
            tree[v].push(u);
        }
        let mut dist = vec![vec![0; n + 1]; n];
        Self::dfs1(0, n + 1, &tree, &mut dist);
        Self::dfs2(0, n + 1, &tree, &mut dist, vec![0; n + 1]);
        dist.iter_mut().for_each(|v| {
            (1..v.len()).for_each(|i| {
                v[i] += v[i - 1];
            });
        });
        dist
    }

    fn dfs1(root: usize, parent: usize, tree: &Vec<Vec<usize>>, dist: &mut Vec<Vec<i32>>) {
        dist[root][0] = 1;
        for &child in &tree[root] {
            if child == parent {
                continue;
            }
            Self::dfs1(child, root, tree, dist);
            for i in 1..dist[root].len() {
                dist[root][i] += dist[child][i - 1];
            }
        }
    }

    fn dfs2(
        root: usize,
        parent: usize,
        tree: &Vec<Vec<usize>>,
        dist: &mut Vec<Vec<i32>>,
        from_parent_dist: Vec<i32>,
    ) {
        for i in 1..dist[root].len() {
            dist[root][i] += from_parent_dist[i - 1];
        }

        dist[root][0] = 1;
        for &child in &tree[root] {
            if child == parent {
                continue;
            }
            let mut v = dist[root].clone();
            for i in 1..dist[root].len() {
                v[i] -= dist[child][i - 1];
            }
            Self::dfs2(child, root, tree, dist, v);
        }
    }
}
