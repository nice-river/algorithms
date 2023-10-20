#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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

use rand::distributions::Slice;

use crate::*;

struct Solution {}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let mut ans = 0;
        Self::dfs(root, distance, &mut ans);
        ans
    }

    fn dfs(root: Rc<RefCell<TreeNode>>, distance: i32, ans: &mut i32) -> HashMap<i32, i32> {
        let node = root.borrow();
        let mut left = HashMap::new();
        if let Some(ch) = node.left.as_ref() {
            left = Self::dfs(ch.clone(), distance, ans);
        }
        let mut right = HashMap::new();
        if let Some(ch) = node.right.as_ref() {
            right = Self::dfs(ch.clone(), distance, ans);
        }
        let mut ret = HashMap::new();
        if left.is_empty() && right.is_empty() {
            ret.insert(1, 1);
            return ret;
        }
        for (a, b) in left.iter() {
            for (c, d) in right.iter() {
                if (a + c) <= distance {
                    *ans += b * d;
                }
            }
        }
        for (a, b) in left.iter() {
            ret.insert(a + 1, *b);
        }
        for (a, b) in right.iter() {
            *ret.entry(*a + 1).or_insert(0) += b;
        }
        ret
    }
}
