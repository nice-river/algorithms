//
// Created by linjiaming on 6/23/2021.
//

class Solution {
public:
	int hammingWeight(uint32_t n) {
		int ans = 0;
		while (n != 0) {
			ans += 1;
			n = n & (n - 1);
		}
		return ans;
	}
};