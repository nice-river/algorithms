from leetcode import ListNode

class Solution:
	def reorderList(self, head: ListNode) -> None:
		"""
		Do not return anything, modify head in-place instead.
		"""
		self.head = head
		self.dfs(head)

	def dfs(self, node):
		if node is None:
			return True
		if self.dfs(node.next):
			if self.head != node and self.head.next != node:
				node.next = self.head.next
				self.head.next = node
				self.head = node.next
				return True
			else:
				node.next = None
				return False
		return False

head = ListNode(1)
head.next = ListNode(2)
head.next.next = ListNode(3)
Solution().reorderList(head)
while head is not None:
	print(head.val, ' ',)
	head = head.next
