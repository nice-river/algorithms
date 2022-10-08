//
// Created by Sinn on 6/5/2021.
//


#include "leetcode.h"


class Solution {
public:
	ListNode* removeElements(ListNode* head, int val) {
		ListNode *nil = new ListNode(-1);
		ListNode *p = nil;
		while (head != nullptr) {
			if (head->val != val) {
				p->next = head;
				p = p->next;
			}
			head = head->next;
		}
		p->next = nullptr;
		return nil->next;
	}
};