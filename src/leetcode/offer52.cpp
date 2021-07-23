//
// Created by linjiaming on 7/21/2021.
//

#include "leetcode.h"

class Solution {
public:
	ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
		ListNode* a = headA;
		ListNode* b = headB;
		while (a != b) {
			a = a ? a->next : headB;
			b = b ? b->next : headA;
		}
		return a;
	}
};
