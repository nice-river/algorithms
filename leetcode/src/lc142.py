from leetcode import ListNode


class Solution:
	def detectCycle(self, head: ListNode) -> ListNode:
		if not head:
			return head
		slow = head
		fast = head
		while fast and fast.next:
			slow = slow.next
			fast = fast.next.next
			if slow == fast:
				fast = head
				while slow != fast:
					slow = slow.next
					fast = fast.next
				return slow
		return None
