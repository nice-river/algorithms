struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let l1 = Some(Box::new(ListNode::new(1)));
		let l2 = Some(Box::new(ListNode::new(1)));
		let res = Solution::add_two_numbers(l1, l2);
		assert_eq!(res.unwrap().val, 2);
	}
}

use crate::leetcode::ListNode;
use std::borrow::Borrow;

impl Solution {
	pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut head = Box::new(ListNode { val: 0, next: None });
		let (mut l1, mut l2, mut carry, mut cur) = (l1, l2, 0, head.as_mut());
		while l1.is_some() || l2.is_some() || carry > 0 {
			let mut num = carry;
			if let Some(mut node) = l1 {
				num += node.val;
				l1 = node.next;
			}
			if let Some(mut node) = l2 {
				num += node.val;
				l2 = node.next;
			}
			carry = num / 10;
			cur.next = Some(Box::new(ListNode { val: num % 10, next: None }));
			cur = cur.next.as_mut().unwrap();
		}
		head.next
	}
}