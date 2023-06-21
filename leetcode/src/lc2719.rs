#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let k = Solution::count("1".to_owned(), "12".to_owned(), 1, 8);
        let ans = 11;
        assert_eq!(k, ans);
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

const MODULE: i64 = 1000000007;

impl Solution {
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let k = num1.as_bytes().iter().map(|&x| (x - b'0') as i32).sum();
        let a = Self::helper(num1, min_sum, max_sum);
        let b = Self::helper(num2, min_sum, max_sum);
        let mut ans = ((b - a) % MODULE + MODULE) % MODULE;
        if min_sum <= k && k <= max_sum {
            ans += 1;
            ans %= MODULE;
        }
        ans as i32
    }

    fn helper(num: String, min_sum: i32, max_sum: i32) -> i64 {
        let max_num = num
            .into_bytes()
            .into_iter()
            .map(|x| (x - b'0') as i32)
            .collect::<Vec<_>>();
        let mut dp = vec![vec![-1i64; 500]; max_num.len()];
        return Self::dfs(
            0,
            &max_num,
            0,
            min_sum as usize,
            max_sum as usize,
            false,
            &mut dp,
        );
    }

    fn dfs(
        idx: usize,
        max_num: &Vec<i32>,
        cur_sum: usize,
        min_sum: usize,
        max_sum: usize,
        is_less: bool,
        dp: &mut Vec<Vec<i64>>,
    ) -> i64 {
        if cur_sum > max_sum {
            return 0;
        }
        if idx == max_num.len() {
            if min_sum <= cur_sum && cur_sum <= max_sum {
                return 1;
            } else {
                return 0;
            }
        }
        if is_less {
            if dp[idx][cur_sum] != -1 {
                return dp[idx][cur_sum];
            }
        }
        let t = if is_less { 9 } else { max_num[idx] };
        let mut m = 0;
        for k in 0..=t {
            let g = Self::dfs(
                idx + 1,
                max_num,
                cur_sum + k as usize,
                min_sum,
                max_sum,
                is_less || k < t,
                dp,
            );
            m += g;
            m %= MODULE;
        }
        if is_less {
            dp[idx][cur_sum] = m;
        }
        m
    }
}
