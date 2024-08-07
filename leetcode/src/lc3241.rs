#[cfg(test)]
mod tests {
    macro_rules! to_vec {
        ([ [$($x:tt),* $(,)?] $(,)?] $(,)?) => {
            vec![vec![ $(to_vec!($x)),* ]]
        };
        ([ [$($x:tt),* $(,)?], $($y:tt),+ $(,)?] $(,)?) => {
            {
                let mut x = vec![vec![$(to_vec!($x)),* ]];
                x.extend([$(to_vec!($y)),+]);
                x
            }
        };
        ([ $($x:expr),* $(,)?] $(,)?) => {
            vec![ $(to_vec!($x)),* ]
        };
        ($x:expr) => {
            $x
        }
    }
    use super::*;

    #[test]
    fn test0() {
        let edges = to_vec!([[2, 4], [0, 1], [2, 3], [0, 2]]);
        let ans = vec![4, 6, 3, 5, 5];
        assert_eq!(Solution::time_taken(edges), ans);
    }

    #[test]
    fn test1() {
        let edges = to_vec!([[0, 1], [0, 2]]);
        let ans = vec![2, 4, 3];
        assert_eq!(Solution::time_taken(edges), ans);
    }
}

use crate::*;

struct Solution {}

use std::collections::BTreeSet;

impl Solution {
    pub fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut tree = vec![vec![]; n];
        let mut weight = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].push(v);
            tree[v].push(u);
            weight[u].push(0);
            weight[v].push(0);
        }
        let mut ans = vec![0; n];
        ans[0] = Self::dfs1(0, n, &tree, &mut weight);
        Self::dfs2(0, n, 0, &tree, &weight, &mut ans);
        ans.into_iter().map(|e| e as i32).collect()
    }

    fn dfs1(
        root: usize,
        parent: usize,
        tree: &Vec<Vec<usize>>,
        weight: &mut Vec<Vec<usize>>,
    ) -> usize {
        let mut ret = 0;
        for (i, &ch) in tree[root].iter().enumerate() {
            if ch == parent {
                continue;
            }
            let v = Self::dfs1(ch, root, tree, weight);
            weight[root][i] = v + 2 - ch % 2;
            ret = ret.max(weight[root][i]);
        }
        ret
    }

    fn dfs2(
        root: usize,
        parent: usize,
        val: usize,
        tree: &Vec<Vec<usize>>,
        weight: &Vec<Vec<usize>>,
        ans: &mut Vec<usize>,
    ) {
        ans[root] = val.max(*weight[root].iter().max().unwrap());
        let mut set = BTreeSet::new();
        for (&ch, &w) in tree[root].iter().zip(weight[root].iter()) {
            set.insert((w, ch));
        }
        for (&ch, &w) in tree[root].iter().zip(weight[root].iter()) {
            if ch == parent {
                continue;
            }
            set.remove(&(w, ch));
            Self::dfs2(
                ch,
                root,
                val.max(set.iter().next_back().unwrap_or(&(0, 0)).0) + 2 - root % 2,
                tree,
                weight,
                ans,
            );
            set.insert((w, ch));
        }
    }
}
