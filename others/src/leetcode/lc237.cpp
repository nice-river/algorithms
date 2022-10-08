//
// Created by linjiaming on 11/2/2021.
//

#include "leetcode.h"

class Solution {
public:
    void deleteNode(ListNode* node) {
        *node = *(node->next);
    }
};