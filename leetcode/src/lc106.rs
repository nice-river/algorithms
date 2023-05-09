struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let in_order = vec![3, 2, 1];
		let post_order = vec![3, 2, 1];
		assert!(Solution::build_tree(in_order, post_order).is_some());
	}
}

use crate::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
	pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		let mut pos_idx = HashMap::new();
		for (i, p) in inorder.iter().enumerate() {
			pos_idx.insert(*p, i);
		}
		Solution::dfs(0 as i32, inorder.len() as i32 - 1, 0 as i32, postorder.len() as i32 - 1, &pos_idx, &inorder, &postorder)
	}

	fn dfs(a: i32, b: i32, c: i32, d: i32, pos_idx: &HashMap<i32, usize>, in_order: &Vec<i32>, post_order: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		if a > b || c > d {
			None
		} else if a == b {
			Some(Rc::new(RefCell::new(TreeNode::new(in_order[a as usize]))))
		} else {
			let mut node = TreeNode::new(post_order[d as usize]);
			let p = *pos_idx.get(&post_order[d as usize]).unwrap() as i32;
			// println!("{} {}", post_order[d], p);
			node.left = Solution::dfs(a, p - 1, c, c + p - a - 1, pos_idx, in_order, post_order);
			node.right = Solution::dfs(p + 1, b, c + p - a, d - 1, pos_idx, in_order, post_order);
			Some(Rc::new(RefCell::new(node)))
		}
	}
}