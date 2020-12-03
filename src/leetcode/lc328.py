# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

from leetcode import ListNode


class Solution:
	def oddEvenList(self, head: ListNode) -> ListNode:
		if head is None:
			return None

		nil = ListNode(-1)
		nil.next = head

		f = head.next

		while f is not None and f.next is not None:
			p = f.next
			f.next = p.next
			p.next = head.next
			head.next = p

			head = head.next
			f = f.next
		return nil.next
