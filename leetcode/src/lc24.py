from leetcode import ListNode

class Solution:
    def swapPairs(self, head: ListNode) -> ListNode:
        nil = ListNode(0)
        nil.next = head
        head = nil
        while head.next and head.next.next:
            p = head.next
            head.next = p.next
            p.next = head.next.next
            head.next.next = p
            head = head.next.next
        return nil.next