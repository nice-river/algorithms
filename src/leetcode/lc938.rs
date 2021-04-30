struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use crate::leetcode::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
		let mut ans = 0;
        if let Some(root) = root {
            let root = root.as_ref().borrow();
            if root.val < low {
				ans += Solution::range_sum_bst(root.right.clone(), low, high);
			} else if root.val > high {
				ans += Solution::range_sum_bst(root.left.clone(), low, high);
			} else {
				ans += Solution::range_sum_bst(root.left.clone(), low, high);
                ans += root.val;
				ans += Solution::range_sum_bst(root.right.clone(), low, high);
			}
		}
		ans
    }
}