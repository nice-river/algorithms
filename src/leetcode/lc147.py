from leetcode import ListNode


class Solution:
	def insertionSortList(self, head: ListNode) -> ListNode:
		if head is None:
			return head
		nil = ListNode(0)
		nil.next = head

		rest = head.next
		head.next = None

		while rest is not None:
			p = rest
			rest = rest.next
			pre, cur = nil, nil.next
			while cur is not None and cur.val <= p.val:
				cur = cur.next
				pre = pre.next
			p.next = cur
			pre.next = p
		return nil.next
