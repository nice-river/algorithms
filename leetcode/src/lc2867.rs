#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 9;
        let edges = to_vec2d([[7,4],[3,4],[5,4],[1,5],[6,4],[9,5],[8,7],[2,8]]);
        let ans = 17;
        assert_eq!(Solution::count_paths(n, edges), ans);
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

use crate::*;

struct Solution {}

impl Solution {
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut tree = vec![vec![]; n + 1];
        for edge in edges {
            tree[edge[0] as usize].push(edge[1] as usize);
            tree[edge[1] as usize].push(edge[0] as usize);
        }
        let mut is_prime = vec![true; n + 1];
        is_prime[1] = false;
        for i in 2..=n {
            if is_prime[i] {
                for k in 2.. {
                    if i * k <= n {
                        is_prime[i * k] = false;
                    } else {
                        break;
                    }
                }
            }
        }
        let mut ans = 0;
        let mut memo = vec![0; n + 1];
        for i in 2..=n {
            if is_prime[i] {
                let mut t = 0;
                for &adj in &tree[i] {
                    if !is_prime[adj] {
                        if memo[adj] == 0 {
                            memo[adj] = Self::dfs(adj, i, &tree, &is_prime);
                        }
                        let c = memo[adj];
                        ans += c;
                        ans += c * t;
                        t += c;
                    }
                }
            }
        }
        ans
    }

    fn dfs(x: usize, p: usize, tree: &Vec<Vec<usize>>, is_prime: &Vec<bool>) -> i64 {
        let mut ret = 1;
        for &adj in &tree[x] {
            if adj != p && !is_prime[adj] {
                ret += Self::dfs(adj, x, tree, is_prime);
            }
        }
        ret
    }
}
