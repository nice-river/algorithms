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

use std::cell::{Ref, RefCell};
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = nums.len();
        Self::dfs(&nums, 0, n)
    }

    fn dfs(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return None;
        }
        if start + 1 == end {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[start]))));
        }
        let mid = (start + end) / 2;
        let mut node = TreeNode::new(nums[mid]);
        node.left = Self::dfs(nums, start, mid);
        node.right = Self::dfs(nums, mid + 1, end);
        Some(Rc::new(RefCell::new(node)))
    }
}
