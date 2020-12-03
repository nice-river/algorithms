from leetcode import ListNode

class Solution:
	def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
		nil = ListNode(0)
		nil.next = head

		def dfs(prev, cur):
			if cur is None:
				return 1
			idx = dfs(cur, cur.next)
			print(cur.val, idx)
			if idx == n:
				prev.next = cur.next
			return idx + 1

		dfs(nil, head)
		return nil.next
