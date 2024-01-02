#include "leetcode.h"

class Solution {
public:
    long long minimumCost(string source, string target, vector<string> &original, vector<string> &changed, vector<int> &cost) {
        using ll = long long;
        const ll MOD = 1000000007;
        std::unordered_map<ll, int> idx;

        auto hash = [](const string &s, int a, int b) {
            ll ret = 0;
            for (int i = a; i < b; ++i) {
                ret = (ret * 27 + s[i] - 'a' + 1) % MOD;
            }
            return ret;
        };

        for (auto &s : original) {
            ll h = hash(s, 0, s.size());
            if (idx.count(h) == 0) {
                auto v = idx.size();
                idx[h] = v;
            }
        }

        for (auto &s : changed) {
            ll h = hash(s, 0, s.size());
            if (idx.count(h) == 0) {
                auto v = idx.size();
                idx[h] = v;
            }
        }

        const ll BLOCK = 1152921504606846975ll;
        auto n = idx.size();
        std::vector<std::vector<ll>> dist;
        dist.resize(n, std::vector<ll>(n, BLOCK));
        for (int i = 0; i < n; ++i) {
            dist[i][i] = 0;
        }
        for (int i = 0; i < original.size(); ++i) {
            ll ha = hash(original[i], 0, original[i].size());
            ll hb = hash(changed[i], 0, changed[i].size());
            int a = idx[ha];
            int b = idx[hb];
            dist[a][b] = min(dist[a][b], (ll)cost[i]);
        }
        for (int k = 0; k < n; ++k) {
            for (int i = 0; i < n; ++i) {
                for (int j = 0; j < n; ++j) {
                    dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
                }
            }
        }
        n = source.size();
        std::vector<std::vector<bool>> same;
        same.resize(n + 1, std::vector<bool>(n + 1, false));
        for (int i = 0; i <= n; ++i) {
            same[i][i] = true;
        }
        for (int i = n - 1; i >= 0; --i) {
            for (int j = i + 1; j <= n; ++j) {
                same[i][j] = source[i] == target[i] && same[i + 1][j];
            }
        }

        std::vector<ll> dp(n + 1, BLOCK);
        dp[n] = 0;

        ll hs = 0;
        ll ht = 0;
        for (int i = n - 1; i >= 0; --i) {
            hs = source[i] - 'a' + 1;
            ht = target[i] - 'a' + 1;
            for (int j = i + 1; j <= n; ++j) {
                auto a = idx.find(hs);
                if (a != idx.end()) {
                    auto b = idx.find(ht);
                    if (b != idx.end()) {
                        dp[i] = min(dp[i], dist[a->second][b->second] + dp[j]);
                    }
                }
                if (same[i][j]) {
                    dp[i] = min(dp[i], dp[j]);
                }
                if (j < n) {
                    hs = (hs * 27 + source[j] - 'a' + 1) % MOD;
                    ht = (ht * 27 + target[j] - 'a' + 1) % MOD;
                }
            }
        }

        if (dp[0] == BLOCK) {
            return -1;
        } else {
            return dp[0];
        }
        return 0;
    }
};

int main() {
    using ll = long long;
    string source = "abb";
    string target = "ccb";
    vector<string> original = {"a", "b"};
    vector<string> changed = {"b", "c"};
    vector<int> cost = {1, 2};
    cout << Solution().minimumCost(source, target, original, changed, cost) << '\n';
    return 0;
}