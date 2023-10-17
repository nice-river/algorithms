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

struct Solution {}

use crate::*;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root.as_ref())
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let node = root.borrow();
            1 + Self::dfs(node.left.as_ref()).max(Self::dfs(node.right.as_ref()))
        } else {
            0
        }
    }
}
