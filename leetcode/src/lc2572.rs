#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![11, 2, 19, 7, 9, 27];
        let ans = 15;
        assert_eq!(Solution::square_free_subsets(nums), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![2, 3, 5, 7, 11];
        let ans = 31;
        assert_eq!(Solution::square_free_subsets(nums), ans);
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
    pub fn square_free_subsets(nums: Vec<i32>) -> i32 {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut arr = vec![];
        'outer: for &num in &nums {
            let mut n = num;
            let mut k = 0;
            for (i, &p) in primes.iter().enumerate() {
                while n % p == 0 {
                    n /= p;
                    if (k & (1 << i)) != 0 {
                        continue 'outer;
                    }
                    k ^= 1 << i;
                }
            }
            arr.push(k);
        }
        if arr.len() == 0 {
            return 0;
        }
        let m = 1100;
        let module = 1000000007i64;
        let mut dp = vec![vec![0; m]; nums.len()];
        dp[0][arr[0]] = 1;
        for i in 1..arr.len() {
            dp[i] = dp[i - 1].clone();
            for j in 0..m {
                if j & arr[i] == 0 {
                    if dp[i - 1][j] != 0 {
                        dp[i][j ^ arr[i]] += dp[i - 1][j];
                        dp[i][j ^ arr[i]] %= module;
                    }
                }
            }
            dp[i][arr[i]] += 1;
        }
        let mut ans = 0i64;
        for j in 0..m {
            ans += dp[arr.len() - 1][j];
            ans %= module;
        }
        ans as i32
    }
}
