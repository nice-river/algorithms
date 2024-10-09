#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 3, 4];
        let queries = vec![0, 2, 2];
        let ans = vec![1, 2, 2];
        assert_eq!(Solution::gcd_values(nums, queries), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![4, 4, 2, 1];
        let queries = vec![5, 3, 1, 0];
        let ans = vec![4, 2, 1, 1];
        assert_eq!(Solution::gcd_values(nums, queries), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 4];
        let queries = vec![0];
        let ans = vec![1];
        assert_eq!(Solution::gcd_values(nums, queries), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let max_n = *nums.iter().max().unwrap() as usize;
        let mut factors = vec![0; max_n + 1];
        for num in nums {
            for i in 1..=(num as f64).sqrt() as i32 {
                if num % i == 0 {
                    factors[i as usize] += 1;
                    if num / i != i {
                        factors[(num / i) as usize] += 1;
                    }
                }
            }
        }
        let mut s = vec![0; max_n + 1];
        for i in (1..=max_n).rev() {
            let mut c = if factors[i] > 0 {
                factors[i] * (factors[i] - 1) / 2
            } else {
                0
            };
            for k in 2.. {
                if i * k >= s.len() {
                    break;
                }
                c -= s[i * k];
            }
            s[i] = c;
        }
        for i in 2..s.len() {
            s[i] += s[i - 1];
        }
        let mut ans = Vec::with_capacity(queries.len());
        for query in queries {
            let mut l = 1;
            let mut r = max_n;
            let query = query as usize + 1;
            while l < r {
                let m = (l + r) / 2;
                if s[m] < query {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            ans.push(r as i32);
        }
        ans
    }
}
