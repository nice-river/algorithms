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
	pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
		if head.is_none() {
			return true;
		}
		let mut head = head.clone();
		let mut arr = vec![];
		while let Some(node) = head {
			arr.push(node.val);
			head = node.next;
		}
		let mut i = 0;
		let mut j = arr.len() - 1;
		while i < j {
			if arr[i] != arr[j] {
				return false;
			}
			i += 1;
			j -= 1;
		}
		true
	}
}