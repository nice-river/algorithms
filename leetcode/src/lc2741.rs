#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![3, 3, 3];
        let ans = 6;
        assert_eq!(Solution::special_perm(nums), ans);
    }

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

struct Solution {}

impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        let module = 1000000007i64;
        let mut ans = 0i64;
        let n = nums.len();
        let mut dp = vec![vec![0; n]; 1 << n];
        for i in 0..n {
            dp[1 << i][i] = 1;
        }
        for i in 1..(1 << n) {
            for j in 0..n {
                if (i & (1 << j)) == 0 {
                    continue;
                }
                for k in 0..n {
                    if (i & (1 << k)) != 0 {
                        continue;
                    }
                    if nums[k] % nums[j] == 0 || nums[j] % nums[k] == 0 {
                        dp[i | (1 << k)][k] += dp[i][j];
                        dp[i | (1 << k)][k] %= module;
                    }
                }
            }
        }
        for i in 0..n {
            ans += dp[(1 << n) - 1][i];
            ans %= module;
        }
        ans as i32
    }
}
