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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&preorder, &inorder)
    }

    fn dfs(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        if preorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        }
        let p = inorder.iter().position(|&x| x == preorder[0]).unwrap();
        let mut node = TreeNode::new(preorder[0]);
        node.left = Self::dfs(&preorder[1..p + 1], &inorder[0..p]);
        node.right = Self::dfs(&preorder[p + 1..], &inorder[p + 1..]);
        Some(Rc::new(RefCell::new(node)))
    }
}
