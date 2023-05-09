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
	pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
		let mut arr = vec![];
		while let Some(mut node) = head {
			head = node.next.take();
			arr.push(node);
		}
		(&mut arr[left as usize - 1..right as usize]).reverse();

		let mut tail = None;
		for mut node in arr.into_iter().rev() {
			node.next = tail;
			tail = Some(node);
		}
		tail
	}
}