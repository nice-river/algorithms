#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 6;
        let edges = to_vec2d([[0, 1], [1, 2], [1, 3], [3, 4], [3, 5]]);
        let price = vec![9, 8, 7, 6, 10, 5];
        let ans = 24;
        assert_eq!(Solution::max_output(n, edges, price), ans);
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

impl Solution {
    pub fn max_output(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut tree = vec![vec![]; n];
        for edge in edges {
            tree[edge[0] as usize].push(edge[1] as usize);
            tree[edge[1] as usize].push(edge[0] as usize);
        }
        let mut min_vals = vec![vec![]; n];
        let mut max_vals = vec![vec![]; n];
        Self::dfs0(0, n, &tree, &mut min_vals, &mut max_vals, &price);
        let mut ans = max_vals[0][0].0 - price[0] as i64;
        let min_val = 0;
        let max_val = 0;
        Self::dfs1(
            0,
            n,
            min_val,
            price[0] as i64,
            &tree,
            &mut min_vals,
            &mut max_vals,
            &price,
            &mut ans,
        );
        ans
    }

    fn dfs0(
        root: usize,
        parent: usize,
        tree: &Vec<Vec<usize>>,
        min_vals: &mut Vec<Vec<(i64, usize)>>,
        max_vals: &mut Vec<Vec<(i64, usize)>>,
        prices: &Vec<i32>,
    ) {
        for &child in tree[root].iter() {
            if child == parent {
                continue;
            }
            Self::dfs0(child, root, tree, min_vals, max_vals, prices);
            let k = min_vals[child][0].0;
            min_vals[root].push((k + prices[root] as i64, child));
            let k = max_vals[child][0].0;
            max_vals[root].push((k + prices[root] as i64, child));
        }
        if min_vals[root].is_empty() {
            min_vals[root].push((prices[root] as i64, root));
            max_vals[root].push((prices[root] as i64, root));
        }
        min_vals[root].sort();
        max_vals[root].sort();
        max_vals[root].reverse();
    }

    fn dfs1(
        root: usize,
        parent: usize,
        min_val: i64,
        max_val: i64,
        tree: &Vec<Vec<usize>>,
        min_vals: &mut Vec<Vec<(i64, usize)>>,
        max_vals: &mut Vec<Vec<(i64, usize)>>,
        prices: &Vec<i32>,
        ans: &mut i64,
    ) {
        *ans = (*ans).max(max_val.max(max_vals[root][0].0) - prices[root] as i64);
        for &child in tree[root].iter() {
            if child == parent {
                continue;
            }
            let p = prices[child] as i64;
            let m = if max_vals[root][0].1 != child {
                (max_val + p).max(max_vals[root][0].0 + p)
            } else if max_vals[root].len() >= 2 {
                (max_val + p).max(max_vals[root][1].0 + p)
            } else {
                max_val + p
            };

            Self::dfs1(
                child, root, min_val, m, tree, min_vals, max_vals, prices, ans,
            );
        }
    }
}
