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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let mut idx = 1i64;
        let mut que = VecDeque::new();
        que.push_back((root.unwrap(), 1));
        while let Some((root, p)) = que.pop_front() {
            if p != idx {
                return false;
            }
            idx += 1;
            let node = root.borrow();
            if let Some(c) = node.left.as_ref() {
                que.push_back((c.clone(), p * 2));
            }
            if let Some(c) = node.right.as_ref() {
                que.push_back((c.clone(), p * 2 + 1));
            }
        }
        true
    }
}
