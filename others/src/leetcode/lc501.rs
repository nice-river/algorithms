struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use crate::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
	pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
		let mut counter = HashMap::new();
		Solution::dfs(root, &mut counter);
		let mut ans = vec![];
		let mut maxi = 0;
		for (k, v) in counter.into_iter() {
			if v > maxi {
				ans.clear();
				ans.push(k);
				maxi = v;
			} else if v == maxi {
				ans.push(k);
			}
		}
		ans
	}

	fn dfs(root: Option<Rc<RefCell<TreeNode>>>, counter: &mut HashMap<i32, i32>) {
		if let Some(root) = root.as_ref() {
			let root = RefCell::borrow(root);
			*counter.entry(root.val).or_insert(0) += 1;
			Solution::dfs(root.left.clone(), counter);
			Solution::dfs(root.right.clone(), counter);
		}
	}
}
