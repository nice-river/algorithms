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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root.as_ref() {
            Self::dfs(root.clone(), None);
        }
    }

    fn dfs(
        mut root: Rc<RefCell<TreeNode>>,
        mut tail: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let right = root.borrow_mut().right.take();
        let left = root.borrow_mut().left.take();
        if let Some(node) = right {
            tail = Self::dfs(node, tail);
        }
        if let Some(node) = left {
            tail = Self::dfs(node, tail);
        }
        root.borrow_mut().right = tail;
        Some(root)
    }
}
