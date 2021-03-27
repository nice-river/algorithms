struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let arr = [1, 1, 2];
		let mut head = None;
		for &node in arr.iter().rev() {
			let mut node = Box::new(ListNode::new(node));
			node.next = head;
			head = Some(node);
		}
		Solution::delete_duplicates(head);
	}
}

use crate::leetcode::leetcode::ListNode;

impl Solution {
	pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut nil = Box::new(ListNode::new(-1));
		if let Some(mut node) = head.take() {
			// append the first node
			head = node.next.take();
			nil.next = Some(node);
		}

		let mut tail = nil.next.as_mut();
		while let Some(mut node) = head.take() {
			head = node.next.take();
			if let Some(elem) = tail.take() {
				// update the tail if the current node val not equals to the tail val
				if elem.val != node.val {
					elem.next = Some(node);
					tail = elem.next.as_mut();
				} else {
					tail.replace(elem);
				}
			}
		}
		nil.next
	}
}
