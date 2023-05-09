struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut arr = Vec::with_capacity(9);
        for i in 0..9 {
			arr.push(Some(TreeNode::new(i as i32)));
		}
		arr[2].as_mut().unwrap().left = Some(Rc::new(RefCell::new(arr[7].take().unwrap())));
		arr[2].as_mut().unwrap().right = Some(Rc::new(RefCell::new(arr[4].take().unwrap())));
		arr[5].as_mut().unwrap().left = Some(Rc::new(RefCell::new(arr[6].take().unwrap())));
		arr[5].as_mut().unwrap().right = Some(Rc::new(RefCell::new(arr[2].take().unwrap())));
		let target = Some(Rc::new(RefCell::new(arr[5].clone().unwrap())));
		arr[1].as_mut().unwrap().left = Some(Rc::new(RefCell::new(arr[0].take().unwrap())));
		arr[1].as_mut().unwrap().right = Some(Rc::new(RefCell::new(arr[8].take().unwrap())));
		arr[3].as_mut().unwrap().left = Some(Rc::new(RefCell::new(arr[5].take().unwrap())));
		arr[3].as_mut().unwrap().right = Some(Rc::new(RefCell::new(arr[1].take().unwrap())));
    let root = Some(Rc::new(RefCell::new(arr[3].take().unwrap())));
		let k = 2;
		let mut ret = Solution::distance_k(root, target, 2);
		ret.sort_unstable();
		let ans = vec![1, 4, 7];
		assert_eq!(ret, ans);
	}

	#[test]
	fn test1() {
		let mut arr = Vec::with_capacity(9);
		for i in 0..9 {
			arr.push(Some(TreeNode::new(i as i32)));
		}
		arr[2].as_mut().unwrap().left = Some(Rc::new(RefCell::new(arr[7].take().unwrap())));
		arr[2].as_mut().unwrap().right = Some(Rc::new(RefCell::new(arr[4].take().unwrap())));
		arr[5].as_mut().unwrap().left = Some(Rc::new(RefCell::new(arr[6].take().unwrap())));
		arr[5].as_mut().unwrap().right = Some(Rc::new(RefCell::new(arr[2].take().unwrap())));
		let target = Some(Rc::new(RefCell::new(arr[5].clone().unwrap())));
		arr[1].as_mut().unwrap().left = Some(Rc::new(RefCell::new(arr[0].take().unwrap())));
		arr[1].as_mut().unwrap().right = Some(Rc::new(RefCell::new(arr[8].take().unwrap())));
		arr[3].as_mut().unwrap().left = Some(Rc::new(RefCell::new(arr[5].take().unwrap())));
		arr[3].as_mut().unwrap().right = Some(Rc::new(RefCell::new(arr[1].take().unwrap())));
		let root = Some(Rc::new(RefCell::new(arr[3].take().unwrap())));
		let k = 5;
		let mut ret = Solution::distance_k(root, target, k);
		ret.sort_unstable();
		let ans = vec![0, 8];
		assert_eq!(ret, ans);
	}
}

use crate::leetcode::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
enum LR {
	Left,
	Right,
}

impl Solution {
	pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
		let mut ans = vec![];
		let mut lrs = vec![];
		let target = target.as_ref().unwrap().as_ref().borrow().val;
		Solution::dfs0(root.as_ref(), target, k, &mut ans, &mut lrs);
        lrs.reverse();
        Solution::dfs1(root.as_ref(), k, &mut ans, &mut lrs, 0);
        ans
	}

	fn dfs0(root: Option<&Rc<RefCell<TreeNode>>>, target: i32, k: i32, ans: &mut Vec<i32>, lrs: &mut Vec<LR>) -> bool {
        if let Some(x) = root {
            let node = x.as_ref().borrow();
			if node.val == target {
                Solution::dfs2(root.clone(), k, ans);
				return true;
			}
			let l = Solution::dfs0(node.left.as_ref(), target, k, ans, lrs);
			let r = Solution::dfs0(node.right.as_ref(), target, k, ans, lrs);
            if l {
				lrs.push(LR::Left);
			}
			if r {
				lrs.push(LR::Right);
			}
            l | r
		} else {
			false
		}
	}

	fn dfs1(root: Option<&Rc<RefCell<TreeNode>>>, k: i32, ans: &mut Vec<i32>, lrs: &mut Vec<LR>, idx: usize) {
        if idx >= lrs.len() {
			return;
		}
		if let Some(root) = root {
			let node = root.as_ref().borrow();
			if k - (lrs.len() - idx) as i32 == 0 {
				ans.push(node.val);
			}
            match lrs[idx] {
				LR::Left => {
					Solution::dfs2(node.right.as_ref(), k - (lrs.len() - idx) as i32 - 1, ans);
					Solution::dfs1(node.left.as_ref(), k, ans, lrs, idx + 1);
				}
				LR::Right => {
					Solution::dfs2(node.left.as_ref(), k - (lrs.len() - idx) as i32 - 1, ans);
					Solution::dfs1(node.right.as_ref(), k, ans, lrs, idx + 1);
				}
			}
		}
	}


	fn dfs2(root: Option<&Rc<RefCell<TreeNode>>>, k: i32, ans: &mut Vec<i32>) {
		if k < 0 {
			return ;
		}
		if let Some(root) = root {
			let node = root.as_ref().borrow();
			if k == 0 {
				ans.push(node.val);
			} else {
                Solution::dfs2(node.left.as_ref(), k - 1, ans);
				Solution::dfs2(node.right.as_ref(), k - 1, ans);
			}
		}
	}
}