class Solution:
    def minimumOperations(self, nums) -> int:
        if len(nums) <= 1:
            return 0
        maps = [{}, {}]
        lens = [0, 0]
        for i in range(len(nums)):
            maps[i % 2][nums[i]] = maps[i % 2].get(nums[i], 0) + 1
            lens[i % 2] += 1
        for i in [0, 1]:
            maps[i] = sorted(maps[i].items(), key=lambda x: x[1], reverse=True)

        if maps[0][0][0] == maps[1][0][0]:
            if len(maps[0]) == 1:
                if len(maps[1]) == 1:
                    ans = min(lens[0], lens[1])
                else:
                    ans = lens[1] - maps[1][1][1]
            elif len(maps[1]) == 1:
                ans = lens[0] - maps[0][1][1]
            else:
                ans = min(lens[0] - maps[0][1][1] + lens[1] - maps[1][0][1],
                          lens[0] - maps[0][0][1] + lens[1] - maps[1][1][1])
        else:
            ans = lens[0] - maps[0][0][1] + lens[1] - maps[1][0][1]
        return ans


nums = [3,1,3,2,4,3]
assert Solution().minimumOperations(nums) == 3