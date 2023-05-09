#include "leetcode.h"

// Definition for a Node.
class Node {
public:
	int val;
	Node* prev;
	Node* next;
	Node* child;
};

class Solution {
public:
	Node* flatten(Node* head) {
		if (head == nullptr) {
			return nullptr;
		}

		Node *nil = new Node();
		Node *p = nil;

		stack<Node*> stk;
		stk.emplace(head);

		while (!stk.empty()) {
			Node *cur = stk.top();
			stk.pop();
			if (cur->next != nullptr) {
				stk.emplace(cur->next);
			}
			if (cur->child != nullptr) {
				stk.emplace(cur->child);
				cur->child = nullptr;
			}
			cur->prev = p;
			p->next = cur;
			p = cur;
		}

		nil->next->prev = nullptr;
		return nil->next;
	}
};