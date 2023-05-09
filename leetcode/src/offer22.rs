struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use crate::leetcode::leetcode::ListNode;

impl Solution {
	pub fn get_kth_from_end(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
		let mut nodes = vec![];
		while let Some(mut node) = head.take() {
			head = node.next.take();
			nodes.push(node);
		}
		let mut ans = None;
		for _ in 0..k {
			let mut node = nodes.pop().unwrap();
			node.next = ans;
			ans = Some(node);
		}
		ans
	}
}
