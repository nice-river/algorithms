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
	pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
		let mut nil = Some(Box::new(ListNode::new(-1)));
		nil.as_mut().unwrap().next = head;
		Solution::dfs(&mut nil, n);
		nil.unwrap().next
	}

	fn dfs(node: &mut Option<Box<ListNode>>, target: i32) -> i32 {
		if let Some(node) = node {
			let ret = Solution::dfs(&mut node.next, target);
			if ret == target {
				let nxt = &node.next.as_ref().unwrap().next;
				node.next = nxt.clone()
			}
			ret + 1
		} else {
			0
		}
	}
}