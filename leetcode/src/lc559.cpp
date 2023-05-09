//
// Created by sinn on 11/21/21.
//

#include "leetcode.h"

class Solution {
public:
    int maxDepth(Node* root) {
        return dfs(root);
    }

    int dfs(Node *root) {
        if (root == nullptr) {
            return 0;
        }
        int ret = 0;
        for (auto &ch : root->children) {
            ret = max(ret, dfs(ch));
        }
        return ret + 1;
    }
};