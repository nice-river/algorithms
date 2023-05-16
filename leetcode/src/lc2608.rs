#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let edges = vec![[0, 1], [2, 0]];
        let edges = edges.into_iter().map(|x| x.to_vec()).collect();
        let ans = -1;
        assert_eq!(Solution::find_shortest_cycle(n, edges), ans);
    }
}

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for (i, edge) in edges.iter().enumerate() {
            graph[edge[0] as usize].push((edge[1] as usize, i));
            graph[edge[1] as usize].push((edge[0] as usize, i));
        }
        let mut ans = i32::MAX;
        let mut vis = vec![-1; n as usize];
        for i in 0..n as usize {
            let mut que = VecDeque::new();
            vis.fill(-1);
            que.push_back((i, 0, edges.len()));
            vis[i] = 0;
            while let Some((cur, s, from)) = que.pop_front() {
                for &(nxt, e) in &graph[cur] {
                    if vis[nxt] != - 1 {
                        if nxt != i && e != from {
                            ans = ans.min(vis[nxt] + s + 1);
                        }
                    } else {
                        vis[nxt] = s + 1;
                        que.push_back((nxt, s + 1, e));
                    }
                }
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
