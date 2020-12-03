from functools import lru_cache


class Solution:
    def combinationSum(self, candidates, target):
        self.candidates = candidates
        self.candidates.sort(reverse=True)
        self.dp = [[None for _ in range(len(candidates))] for _ in range(target + 1)]
        ret = self.dfs(target, 0)
        if ret == -1:
            return []
        ans = []
        for one in ret:
            ans.append([])
            for x, y in one:
                for _ in range(y):
                    ans[-1].append(x)
        return ans

    def dfs(self, target, idx):
        if self.dp[target][idx] is not None:
            return self.dp[target][idx]
        if target == 0:
            return [[]]
        if idx == len(self.candidates) - 1:
            if target % self.candidates[-1] == 0:
                self.dp[target][idx] = [[(self.candidates[-1], target // self.candidates[-1])]]
            else:
                self.dp[target][idx] = -1
            return self.dp[target][idx]
        ans = []
        for i in range(0, target // self.candidates[idx] + 1):
            a = target - self.candidates[idx] * i
            ret = self.dfs(a, idx + 1)
            if ret != -1:
                if i == 0:
                    ans = ret[:]
                else:
                    for r in ret:
                        ans.append(r[:])
                        ans[-1].append((self.candidates[idx], i))
        if not ans:
            self.dp[target][idx] = -1
            ans = -1
        else:
            self.dp[target][idx] = ans
        return ans


print(len(Solution().combinationSum(list(range(1, 31)), 500)))

[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30]