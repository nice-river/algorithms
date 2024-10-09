#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let edges = to_vec!([[0, 1], [0, 2], [1, 3], [2, 3]]);
        dbg!(Solution::construct_grid_layout(n, edges));

        let n = 5;
        let edges = to_vec!([[0, 1], [1, 3], [2, 3], [2, 4]]);
        dbg!(Solution::construct_grid_layout(n, edges));

        let n = 9;
        let edges = to_vec!([
            [0, 1],
            [0, 4],
            [0, 5],
            [1, 7],
            [2, 3],
            [2, 4],
            [2, 5],
            [3, 6],
            [4, 6],
            [4, 7],
            [6, 8],
            [7, 8]
        ]);
        dbg!(Solution::construct_grid_layout(n, edges));
    }

    #[test]
    fn test1() {
        let n = 4;
        let edges = to_vec!([[0, 1], [0, 2], [1, 3]]);
        dbg!(Solution::construct_grid_layout(n, edges));
    }
}

use crate::*;

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn construct_grid_layout(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        for adj in graph.iter() {
            if adj.is_empty() {
                return vec![vec![0]];
            } else if adj.len() == 1 {
                return Self::construce1d(&graph);
            }
        }
        Self::construce2d(&graph)
    }

    fn construce1d(graph: &Vec<Vec<usize>>) -> Vec<Vec<i32>> {
        let mut arr = vec![];
        for (i, adj) in graph.iter().enumerate() {
            if adj.len() == 1 {
                arr.push(i);
                arr.push(adj[0]);
                break;
            }
        }
        loop {
            let v = *arr.last().unwrap();
            let mut done = true;
            for &t in &graph[v] {
                if t != arr[arr.len() - 2] {
                    arr.push(t);
                    done = false;
		    break;
                }
            }
            if done {
                break;
            }
        }
        vec![arr.into_iter().map(|v| v as i32).collect()]
    }

    fn construce2d(graph: &Vec<Vec<usize>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![], vec![]];
        for (i, adj) in graph.iter().enumerate() {
            if adj.len() == 2 {
                ret[0].push(i);
                ret[0].push(adj[0]);
                ret[1].push(adj[1]);
                break;
            }
        }
        loop {
            let a = *ret[0].last().unwrap();
            let b = *ret[1].last().unwrap();

            let set_a: HashSet<&usize> = HashSet::from_iter(graph[a].iter());
            let set_b: HashSet<&usize> = HashSet::from_iter(graph[b].iter());
            for &&x in set_a.intersection(&set_b) {
                if x != ret[0][ret[0].len() - 2] {
                    ret[1].push(x);
                }
            }
            if graph[a].len() == 2 {
                break;
            }

            let c = ret[0][ret[0].len() - 2];
            let b = *ret[1].last().unwrap();
            for &adj in &graph[a] {
                if adj != c && adj != b {
                    ret[0].push(adj);
                }
            }
        }
        let m = ret[0].len();
        loop {
            let n = ret.len();
            let mut row = vec![];
            for i in 0..m {
                let a = ret[n - 1][i];
                let mut nei = vec![ret[n - 2][i]];
                if i > 0 {
                    nei.push(ret[n - 1][i - 1])
                }
                if i + 1 < m {
                    nei.push(ret[n - 1][i + 1]);
                }
                if graph[a].len() == nei.len() {
                    break;
                }
                for adj in &graph[a] {
                    if !nei.contains(adj) {
                        row.push(*adj);
                    }
                }
            }
            if row.is_empty() {
                break;
            } else {
                ret.push(row);
            }
        }

        ret.into_iter()
            .map(|v| v.into_iter().map(|x| x as i32).collect())
            .collect()
    }
}
