#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 6;
        let edges = vec![[0, 1], [0, 2], [1, 2], [3, 4]];
        let edges = edges.into_iter().map(|x| x.to_vec()).collect();
        let ans = 3;
        assert_eq!(Solution::count_complete_components(n, edges), ans);
        // ass
        // 6
        // [[0,1],[0,2],[1,2],[3,4],[3,5]]
    }
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut graph = vec![HashSet::<usize>::new(); n as usize + 1];
        for edge in edges {
            graph[edge[0] as usize].insert(edge[1] as usize);
            graph[edge[1] as usize].insert(edge[0] as usize);
        }
        let mut comp = vec![0; n as usize + 1];
        let mut c = 1;
        for i in 0..n as usize {
            if comp[i] == 0 {
                Self::dfs(i, &mut comp, c, &graph);
                c += 1;
            }
        }
        let mut g = vec![vec![]; c + 1];
        for i in 0..n as usize {
            g[comp[i]].push(i);
        }
        for i in 1..c {
            let mut f = true;
            'check: for a in 0..g[i].len() {
                for b in a + 1..g[i].len() {
                    if !graph[g[i][a]].contains(&g[i][b]) {
                        f = false;
                        break 'check;
                    }
                }
            }
            if f {
                ans += 1;
            }
        }
        ans
    }

    fn dfs(root: usize, comp: &mut Vec<usize>, c: usize, graph: &Vec<HashSet<usize>>) {
        comp[root] = c;
        for &adj in graph[root].iter() {
            if comp[adj] == 0 {
                Self::dfs(adj, comp, c, graph);
            }
        }
    }
}
