#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test0() {
	}
}

struct Solution {}

use crate::leetcode::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
	    let mut arr = vec![];
	    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
		    if let Some(x) = node {
			    let x = x.as_ref().borrow();
			    dfs(x.left.as_ref(), arr);
			    arr.push(x.val);
			    dfs(x.right.as_ref(), arr);
		    }
	    }
	    dfs(root.as_ref(), &mut arr);
	    arr[k as usize - 1]
    }
}