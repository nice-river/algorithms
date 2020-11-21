struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use crate::leetcode::ListNode;

impl Solution {
	pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut head = head.clone();
		let mut arr = vec![];
		while head.is_some() {
			let inner = head.unwrap();
			arr.push(inner.val);
			head = inner.next;
		}
		arr.sort();
		head = None;
		for v in arr.into_iter().rev() {
			let mut node = ListNode::new(v);
			node.next = head;
			head = Some(Box::new(node));
		}
		head
	}
}