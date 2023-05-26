#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let edges = to_vec2d([[0, 1], [1, 2], [1, 3], [4, 2]]);
        let guesses = to_vec2d([[1, 3], [0, 1], [1, 0], [2, 4]]);
        let k = 3;
        let ans = 3;
        assert_eq!(Solution::root_count(edges, guesses, k), ans);
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

use std::collections::HashSet;

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut g = HashSet::new();
        guesses.into_iter().for_each(|e| {
            g.insert((e[0] as usize, e[1] as usize));
            ()
        });
        let guesses = g;

        let n = edges.len() + 1;
        let mut tree = vec![vec![]; n];
        for edge in &edges {
            tree[edge[0] as usize].push(edge[1] as usize);
            tree[edge[1] as usize].push(edge[0] as usize);
        }
        let x = Self::dfs0(0, n, &tree, &guesses);
        let mut ans = 0;
        Self::dfs1(0, tree.len(), &tree, &guesses, x, k, &mut ans);
        ans
    }

    fn dfs0(
        root: usize,
        parent: usize,
        tree: &Vec<Vec<usize>>,
        guesses: &HashSet<(usize, usize)>,
    ) -> i32 {
        let mut ret = 0;
        for &ch in tree[root].iter() {
            if ch != parent {
                ret += Self::dfs0(ch, root, tree, guesses);
                if guesses.contains(&(root, ch)) {
                    ret += 1;
                }
            }
        }
        ret
    }

    fn dfs1(
        root: usize,
        parent: usize,
        tree: &Vec<Vec<usize>>,
        guesses: &HashSet<(usize, usize)>,
        cur_k: i32,
        target: i32,
        ans: &mut i32,
    ) {
        if cur_k >= target {
            *ans += 1;
        }
        for &ch in tree[root].iter() {
            if ch != parent {
                let mut k = cur_k;
                if guesses.contains(&(root, ch)) {
                    k -= 1;
                }
                if guesses.contains(&(ch, root)) {
                    k += 1;
                }
                Self::dfs1(ch, root, tree, guesses, k, target, ans);
            }
        }
    }
}
