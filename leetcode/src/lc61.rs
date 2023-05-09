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
	pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
		if head.is_none() {
			return None;
		}
		let mut nodes = vec![];
		while let Some(mut node) = head.take() {
			head = node.next.take();
			nodes.push(node);
		}
		let k = (k as usize) % nodes.len();
		nodes.rotate_right(k);
		let mut head = None;
		for mut node in nodes.into_iter().rev() {
			node.next = head;
			head = Some(node);
		}
		head
	}
}