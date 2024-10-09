#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let k = 1;
        let invocations = to_vec!([[1, 2], [0, 1], [3, 2]]);
        let ans = vec![0, 1, 2, 3];
        assert_eq!(Solution::remaining_methods(n, k, invocations), ans);
    }
}

use crate::*;

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut prior = vec![vec![]; n];
        for invocation in invocations {
            let u = invocation[0] as usize;
            let v = invocation[1] as usize;
            graph[u].push(v);
            prior[v].push(u);
        }
        let mut wrong = HashSet::new();
        wrong.insert(k as usize);
        Self::dfs(k as usize, &graph, &mut wrong);
        for &v in wrong.iter() {
            for p in &prior[v] {
                if !wrong.contains(p) {
                    return (0..n as i32).collect::<Vec<_>>();
                }
            }
        }
        (0..n as i32)
            .filter(|x| !wrong.contains(&(*x as usize)))
            .collect::<Vec<_>>()
    }

    fn dfs(node: usize, graph: &Vec<Vec<usize>>, wrong: &mut HashSet<usize>) {
        for nxt in &graph[node] {
            if !wrong.contains(nxt) {
                wrong.insert(*nxt);
                Self::dfs(*nxt, graph, wrong);
            }
        }
    }
}
