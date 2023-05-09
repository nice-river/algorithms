//
// Created by Sinn on 6/4/2021.
//

#include "leetcode.h"

class Solution {
public:
	ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
		ListNode *pa = headA, *pb = headB;
		while (pa != pb) {
			pa = (pa == nullptr) ? headB : pa->next;
			pb = (pb == nullptr) ? headA : pb->next;
		}
		return pa;
	}
};
