#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let edges = to_vec!([[0, 1], [0, 2], [0, 3]]);
        let x = 1;
        let y = 2;
        let z = 3;
        let ans = 3;
        assert_eq!(Solution::special_nodes(n, edges, x, y, z), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn special_nodes(n: i32, edges: Vec<Vec<i32>>, x: i32, y: i32, z: i32) -> i32 {
        let n = n as usize;
        let mut tree = vec![vec![]; n];
        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].push(v);
            tree[v].push(u);
        }
        let mut dx = vec![0; n];
        let mut dy = vec![0; n];
        let mut dz = vec![0; n];
        Solution::dfs(x as usize, n, 0, &tree, &mut dx);
        Solution::dfs(y as usize, n, 0, &tree, &mut dy);
        Solution::dfs(z as usize, n, 0, &tree, &mut dz);
        let mut ans = 0;
        for ((a, b), c) in dx.into_iter().zip(dy.into_iter()).zip(dz.into_iter()) {
            let mut tri = vec![a, b, c];
            tri.sort();
            if tri[0] * tri[0] + tri[1] * tri[1] == tri[2] * tri[2] {
                ans += 1;
            }
        }
        ans
    }

    fn dfs(root: usize, parent: usize, step: usize, tree: &Vec<Vec<usize>>, dis: &mut Vec<usize>) {
        dis[root] = step;
        for &child in tree[root].iter() {
            if child != parent {
                Solution::dfs(child, root, step + 1, tree, dis);
            }
        }
    }
}
