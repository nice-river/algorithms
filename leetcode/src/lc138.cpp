//
// Created by linjiaming on 7/22/2021.
//

#include "leetcode.h"

class Node {
public:
	int val;
	Node* next;
	Node* random;

	Node(int _val) {
		val = _val;
		next = NULL;
		random = NULL;
	}
};

class Solution {
public:
	Node* copyRandomList(Node* head) {
		unordered_map<Node*, Node*> map;
		Node* nil = new Node(-1);
		Node* p = nil;
		while (head != nullptr) {
			if (map.find(head) == map.end()) {
				map.emplace(head, new Node(head->val));
			}
			Node* n = map[head];
			if (head->random != nullptr) {
				if (map.find(head->random) == map.end()) {
					map.emplace(head->random, new Node(head->random->val));
				}
				n->random = map[head->random];
			}
			p->next = n;
			p = p->next;
			head = head->next;
		}
		return nil->next;
	}
};