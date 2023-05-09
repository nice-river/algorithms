struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test0() {
	}
}

use crate::leetcode::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let root = root.unwrap();
        let mut que = VecDeque::new();
        que.push_back(root);
        let mut level = 1;
        while !que.is_empty() {
            let mut val = None;
            for i in 0..que.len() {
                let cur = que.pop_front().unwrap();
                let cur = cur.borrow();
                if i != 0 {
                    if level == 1 {
                        if cur.val <= val.unwrap() {
                            return false;
                        }
                    } else {
                        if cur.val >= val.unwrap() {
                            return false;
                        }
                    }
                }
                if cur.val % 2 != level {
                    return false;
                }
                val = Some(cur.val);
                if cur.left.is_some() {
                    que.push_back(cur.left.as_ref().unwrap().clone());
                }
                if cur.right.is_some() {
                    que.push_back(cur.right.as_ref().unwrap().clone());
                }
            }
            level ^= 1;
        }

        true
    }
}