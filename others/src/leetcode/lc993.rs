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
use std::collections::HashMap;

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let map = &mut HashMap::new();
        Solution::dfs(root, map, -1, 0);
		let x = map.get(&x).unwrap();
		let y = map.get(&y).unwrap();
        x.0 == y.0 && x.1 != y.1
    }

	fn dfs(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<i32, (i32, i32)>, parent: i32, level: i32) {
		if let Some(node) = root {
			let node = node.as_ref().borrow();
            map.insert(node.val, (level, parent));
			Solution::dfs(node.left.clone(), map, node.val, level + 1);
			Solution::dfs(node.right.clone(), map, node.val, level + 1);
		}
	}
}