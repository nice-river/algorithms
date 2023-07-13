#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int maximumSum(vector<int> &arr) {
        int ans = INT32_MIN;
        for (int a : arr) {
            ans = max(ans, a);
        }
        if (ans < 0) {
            return ans;
        }
        vector<vector<int>> dp(arr.size(), vector<int>(2, 0));
        dp[0][0] = arr[0];
        for (int i = 1; i < arr.size(); ++i) {
            dp[i][0] = dp[i - 1][0] + arr[i];
            dp[i][0] = max(dp[i][0], arr[i]);
            dp[i][1] = dp[i - 1][1] + arr[i];
            dp[i][1] = max(dp[i][1], dp[i - 1][0]);
            ans = max({ans, dp[i][0], dp[i][1]});
        }
        return ans;
    }
};

int main() {
    Solution sol;
    vector<int> arr({1, -2, 0, 3});
    sol.maximumSum(arr);
    return 0;
}