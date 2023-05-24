#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 5;
        let edges = to_vec2d([[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]]);
        let source = 0;
        let destination = 1;
        let target = 5;
        let ans = to_vec2d([[4, 1, 1], [2, 0, 1], [0, 3, 3], [4, 3, 1]]);
        assert_eq!(
            Solution::modified_graph_edges(n, edges, source, destination, target),
            ans
        );
    }

    #[test]
    fn test1() {
        let n = 5;
        let edges = to_vec2d([
            [3, 0, 1],
            [2, 1, -1],
            [2, 3, 6],
            [4, 2, 6],
            [1, 3, 2],
            [2, 0, 7],
            [0, 4, 10],
            [0, 1, 6],
        ]);
        let source = 1;
        let destination = 4;
        let target = 14;
        let ans: Vec<Vec<i32>> = to_vec2d::<i32, [_; 0], Vec<_>>([]);
        assert_eq!(
            Solution::modified_graph_edges(n, edges, source, destination, target),
            ans,
        );
    }

    #[test]
    fn test2() {
        let n = 5;
        let edges = to_vec2d([
            [1, 4, 1],
            [2, 4, -1],
            [3, 0, 2],
            [0, 4, -1],
            [1, 3, 10],
            [1, 0, 10],
        ]);
        let source = 0;
        let destination = 2;
        let target = 15;
        let ans: Vec<Vec<i32>> = to_vec2d([
            [1, 4, 1],
            [2, 4, 4],
            [3, 0, 2],
            [0, 4, 14],
            [1, 3, 10],
            [1, 0, 10],
        ]);
        assert_eq!(
            Solution::modified_graph_edges(n, edges, source, destination, target),
            ans,
        );
    }

    #[test]
    fn test3() {
        let n = 4;
        let edges = to_vec2d([[0, 1, -1], [2, 0, 2], [3, 2, 6], [2, 1, 10], [3, 0, -1]]);
        let source = 1;
        let destination = 3;
        let target = 12;
        let ans = to_vec2d([[0, 1, 11], [2, 0, 2], [3, 2, 6], [2, 1, 10], [3, 0, 1]]);
        assert_eq!(
            Solution::modified_graph_edges(n, edges, source, destination, target),
            ans,
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

struct Solution {}

use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
struct Node {
    val: i32,
    idx: usize,
    edges_idx: Vec<usize>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        Some(if self.val < other.val {
            Ordering::Greater
        } else {
            Ordering::Less
        })
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Node {
    fn new(idx: usize, val: i32, edges_idx: Vec<usize>) -> Self {
        Self {
            idx,
            val,
            edges_idx,
        }
    }
}

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;
        let target = target;
        let mut graph = vec![vec![]; n];

        let mut s = vec![true; edges.len()];

        for (i, edge) in edges.iter_mut().enumerate() {
            graph[edge[0] as usize].push((edge[1] as usize, i));
            graph[edge[1] as usize].push((edge[0] as usize, i));
            if edge[2] == -1 {
                edge[2] = target + 1;
            } else {
                s[i] = false;
            }
        }
        if let Some(node) = Self::shortest(&graph, &edges, source, destination, &s) {
            if node.val < target {
                return vec![];
            }
        } else {
            return vec![];
        }
        for (i, &v) in s.iter().enumerate() {
            if v {
                edges[i][2] = 1;
            }
        }
        if let Some(node) = Self::shortest(&graph, &edges, source, destination, &s) {
            if node.val > target {
                return vec![];
            }
        } else {
            return vec![];
        }

        loop {
            if let Some(node) = Self::shortest(&graph, &edges, source, destination, &s) {
                if node.val < target {
                    if node.edges_idx.is_empty() {
                        edges.clear();
                        break;
                    } else {
                        let e = node.edges_idx[0];
                        edges[e][2] = target - node.val + 1;
                        s[e] = false;
                    }
                } else if node.val == target {
                    break;
                } else {
                    edges.clear();
                    break;
                }
            } else {
                edges.clear();
                break;
            }
        }
        edges
    }

    fn shortest(
        graph: &Vec<Vec<(usize, usize)>>,
        edges: &Vec<Vec<i32>>,
        src: usize,
        dest: usize,
        s: &Vec<bool>,
    ) -> Option<Node> {
        let mut heap = BinaryHeap::new();
        heap.push(Node::new(src, 0, vec![]));
        let mut dp = vec![-1; graph.len()];
        dp[src] = 0;
        let mut mini_node = None;
        while let Some(node) = heap.pop() {
            if node.idx == dest {
                mini_node = Some(node);
                break;
            }
            if node.val > dp[node.idx] {
                continue;
            }
            for &(adj, edge_idx) in graph[node.idx].iter() {
                let mut node = node.clone();
                let k = node.val + edges[edge_idx][2];
                if s[edge_idx] {
                    node.edges_idx.push(edge_idx);
                }
                if dp[adj] == -1 || k < dp[adj] {
                    node.idx = adj;
                    node.val = k;
                    dp[adj] = k;
                    heap.push(node.clone());
                }
            }
        }
        mini_node
    }
}
