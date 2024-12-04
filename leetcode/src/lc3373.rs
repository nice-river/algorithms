#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let edges1 = to_vec!([[0, 1], [0, 2], [0, 3], [0, 4]]);
        let edges2 = to_vec!([[0, 1], [1, 2], [2, 3]]);
        let ans = vec![3, 6, 6, 6, 6];
        assert_eq!(Solution::max_target_nodes(edges1, edges2), ans);
    }

    #[test]
    fn test1() {
        let edges1 = to_vec!([[0, 1], [0, 2], [2, 3], [2, 4]]);
        let edges2 = to_vec!([[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]]);
        let ans = vec![8, 7, 7, 8, 8];
        assert_eq!(Solution::max_target_nodes(edges1, edges2), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let dist1 = Self::build(edges1);
        let dist2 = Self::build(edges2);
        let mut ans = Vec::with_capacity(dist1.len());
        let mut v = [0; 2];
        dist2.iter().for_each(|d| {
            v[0] = v[0].max(d[0]);
            v[1] = v[1].max(d[1]);
        });
        if dist1.len() == 1 {
            ans.push(dist1[0][0] + v[1]);
        }
        (0..dist1.len()).for_each(|i| {
            ans.push(dist1[i][0] + v[0].max(v[1]));
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
        let mut dist = vec![vec![0; 2]; n];
        Self::dfs1(0, n + 1, &tree, &mut dist);
        Self::dfs2(0, n + 1, &tree, &mut dist, vec![0; n + 1]);
        dist
    }

    fn dfs1(root: usize, parent: usize, tree: &Vec<Vec<usize>>, dist: &mut Vec<Vec<i32>>) {
        dist[root][0] = 1;
        for &child in &tree[root] {
            if child == parent {
                continue;
            }
            Self::dfs1(child, root, tree, dist);
            dist[root][0] += dist[child][1];
            dist[root][1] += dist[child][0];
        }
    }

    fn dfs2(
        root: usize,
        parent: usize,
        tree: &Vec<Vec<usize>>,
        dist: &mut Vec<Vec<i32>>,
        from_parent_dist: Vec<i32>,
    ) {
        dist[root][0] += from_parent_dist[1];
        dist[root][1] += from_parent_dist[0];
        for &child in &tree[root] {
            if child == parent {
                continue;
            }
            let mut v = dist[root].clone();
            v[0] -= dist[child][1];
            v[1] -= dist[child][0];
            Self::dfs2(child, root, tree, dist, v);
        }
    }
}
