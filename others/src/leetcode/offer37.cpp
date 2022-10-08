//
// Created by linjiaming on 6/30/2021.
//


#include "leetcode.h"

class Codec {
public:

	// Encodes a tree to a single string.
	string serialize(TreeNode* root) {
		vector<string> arr;
		queue<TreeNode*> que;
		que.emplace(root);
		while (!que.empty()) {
			TreeNode *node = que.front();
			que.pop();
			if (node == nullptr) {
				arr.emplace_back("#");
				continue;
			}
			arr.emplace_back(to_string(node->val));
			que.emplace(node->left);
			que.emplace(node->right);
		}
		stringstream ss;
		copy(arr.begin(), arr.end(), ostream_iterator<string>(ss, ","));
		return ss.str();
	}

	// Decodes your encoded data to tree.
	TreeNode* deserialize(string data) {
		vector<string> arr;
		auto start = 0;
		auto end = data.find(",");
		while (end != string::npos) {
			arr.emplace_back(data.substr(start, end - start));
			start = end + 1;
			end = data.find(",", start);
		}
		arr.emplace_back(data.substr(start, end));
		auto pos = 0;
		TreeNode* node = nullptr;
		queue<TreeNode**> que;
		que.emplace(&node);
		while (!que.empty()) {
			auto cur = que.front(); que.pop();
			if (arr[pos] != "#") {
				*cur = new TreeNode(stoi(arr[pos]));
				que.push(&(*cur)->left);
				que.push(&(*cur)->right);
			}
			pos += 1;
		}
		return node;
	}
};


int main() {
	TreeNode root(1);
	TreeNode left(2);
	TreeNode right(3);
	root.left = &left;
	root.right = &right;
	Codec codec;
	TreeNode* new_root = codec.deserialize(codec.serialize(&root));
	cout << codec.serialize(new_root) << "\n";
	return 0;
}