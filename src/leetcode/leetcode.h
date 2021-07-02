//
// Created by Sinn on 6/4/2021.
//

#ifndef CP_IN_RUST_LEETCODE_H
#define CP_IN_RUST_LEETCODE_H

#include <string>
#include <vector>
#include <queue>
#include <stack>
#include <algorithm>
#include <iostream>
#include <sstream>
using namespace std;

//Definition for singly-linked list.
struct ListNode {
	int val;
	ListNode *next;
	ListNode(int x) : val(x), next(nullptr) {}
};


// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};


#endif //CP_IN_RUST_LEETCODE_H
