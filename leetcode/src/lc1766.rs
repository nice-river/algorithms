#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 3, 3, 2];
        let edges = to_vec2d([[0, 1], [1, 2], [1, 3]]);
        let ans = vec![-1, 0, 0, 1];
        assert_eq!(Solution::get_coprimes(nums, edges), ans);
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

const N: usize = 50;

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = vec![vec![]; N + 1];
        let mut tree = vec![vec![]; nums.len()];
        for edge in edges {
            tree[edge[0] as usize].push(edge[1] as usize);
            tree[edge[1] as usize].push(edge[0] as usize);
        }
        let mut ans = vec![-1; nums.len()];
        Self::dfs(0, nums.len(), &nums, &tree, &mut arr, 1, &mut ans);
        ans
    }

    fn dfs(
        root: usize,
        parent: usize,
        nums: &Vec<i32>,
        tree: &Vec<Vec<usize>>,
        arr: &mut Vec<Vec<(usize, usize)>>,
        layer: usize,
        ans: &mut Vec<i32>,
    ) {
        let mut cand = (0, nums.len() + 1);
        (1..=N).for_each(|i| {
            if let Some(&v) = arr[i].last() {
                if Self::gcd(nums[v.1], nums[root]) == 1 {
                    cand = cand.max(v);
                }
            }
        });
        if cand.0 != 0 {
            ans[root] = cand.1 as i32;
        }
        arr[nums[root] as usize].push((layer, root));
        for &ch in &tree[root] {
            if ch == parent {
                continue;
            }
            Self::dfs(ch, root, nums, tree, arr, layer + 1, ans);
        }
        arr[nums[root] as usize].pop();
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }
}


