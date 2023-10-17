#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let parents = vec![-1, 0, 1, 0, 3, 3];
        let nums = vec![5, 4, 6, 2, 1, 3];
        let ans = vec![7, 1, 1, 4, 2, 1];
        assert_eq!(Solution::smallest_missing_value_subtree(parents, nums), ans);
    }

    #[test]
    fn test1() {
        let parents = vec![-1, 0, 0, 2];
        let nums = vec![1, 2, 3, 4];
        let ans = vec![5, 1, 1, 1];
        assert_eq!(Solution::smallest_missing_value_subtree(parents, nums), ans);
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

use std::collections::HashMap;

impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut ans = vec![1; nums.len()];
        for (i, num) in nums.into_iter().enumerate() {
            map.insert(num, i);
        }
        if !map.contains_key(&1) {
            return ans;
        }
        let mut arr = vec![*map.get(&1).unwrap() as i32];
        while parents[arr[arr.len() - 1] as usize] != -1 {
            arr.push(parents[arr[arr.len() - 1] as usize]);
        }
        let mut tree = vec![vec![]; ans.len()];
        for (i, &p) in parents.iter().enumerate() {
            if p != -1 {
                tree[p as usize].push(i);
            }
        }
        let mut marks = vec![ans.len(); ans.len()];
        for (i, &root) in arr.iter().enumerate() {
            Self::mark(i, root as usize, &tree, &mut marks)
        }
        let mut p = 0;
        for i in 2.. {
            if p == arr.len() {
                break;
            }
            if let Some(&t) = map.get(&i) {
                let k = marks[t];
                if k > p {
                    for j in p..k {
                        ans[arr[j] as usize] = i;
                    }
                    p = k;
                }
            } else {
                for j in p..arr.len() {
                    ans[arr[j] as usize] = i;
                }
                break;
            }
        }
        ans
    }

    fn mark(p: usize, root: usize, tree: &Vec<Vec<usize>>, marks: &mut Vec<usize>) {
        if marks[root] != marks.len() {
            return;
        }
        marks[root] = p;
        for &child in &tree[root] {
            Self::mark(p, child, tree, marks)
        }
    }
}
