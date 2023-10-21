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

use crate::*;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let _ = Self::dfs(root.as_ref(), 0, &mut ans);
        ans
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, depth: i32, ans: &mut i32) -> i32 {
        if let Some(root) = root {
            let node = root.borrow();
            let t = Self::dfs(node.left.as_ref(), depth + 1, ans);
            let g = Self::dfs(node.right.as_ref(), depth + 1, ans);
            *ans = (*ans).max(depth + t.max(g));
            *ans = (*ans).max(t + g);
            1 + t.max(g)
        } else {
            0
        }
    }
}
