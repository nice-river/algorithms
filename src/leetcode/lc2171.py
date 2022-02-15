class Solution:
    def minimumRemoval(self, beans) -> int:
        beans.sort()
        tot = sum(beans)
        ans = tot
        i = 0
        su = 0
        while i < len(beans):
            j = i + 1
            while j < len(beans) and beans[j] == beans[i]:
                j = j + 1
            t = (j - i) * beans[i]
            ans = min(ans, su + tot - su - t - (len(beans) - j) * beans[i])
            su += t
            i = j
        return ans


beans = [4, 1, 6, 5]
assert Solution().minimumRemoval(beans) == 4