from leetcode.leetcode import ListNode


class Solution:
	def partition(self, head: ListNode, x: int) -> ListNode:
		nil = ListNode(x)
		nil.next = head
		pre = nil
		cur = head
		mark = nil
		while cur is not None:
			if cur.val < x:
				if mark.next != cur:
					pre.next = cur.next
					cur.next = mark.next
					mark.next = cur
					cur = pre.next
				else:
					cur = cur.next
					pre = pre.next
				mark = mark.next
			else:
				cur = cur.next
				pre = pre.next

		return nil.next
