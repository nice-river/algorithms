//
// Created by Sinn on 6/4/2021.
//

#ifndef LEETCODE_H
#define LEETCODE_H

#include <string>
#include <vector>
#include <queue>
#include <stack>
#include <algorithm>
#include <iostream>
#include <sstream>
#include <unordered_map>
#include <unordered_set>
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

/*
 * Below is the interface for Iterator, which is already defined for you.
 * **DO NOT** modify the interface for Iterator.
*/
class Iterator {
    struct Data;
    Data* data;
public:
    Iterator(const vector<int>& nums) {}
    Iterator(const Iterator& iter) {}

    // Returns the next element in the iteration.
    int next();

    // Returns true if the iteration has more elements.
    bool hasNext() const;
};


#endif //LEETCODE_H
