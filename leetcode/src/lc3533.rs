#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![3, 12, 45];
        let k = 5;
        let ans = vec![3, 12, 45];
        assert_eq!(Solution::concatenated_divisibility(nums, k), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![10, 5];
        let k = 10;
        let ans = vec![5, 10];
        assert_eq!(Solution::concatenated_divisibility(nums, k), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3];
        let k = 5;
        let ans = vec![];
        assert_eq!(Solution::concatenated_divisibility(nums, k), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![23, 13];
        let k = 3;
        let ans = vec![13, 23];
        assert_eq!(Solution::concatenated_divisibility(nums, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn concatenated_divisibility(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();

        let mut helper0 = vec![0; n];
        for (i, num) in nums.iter().enumerate() {
            let mut g = *num;
            let mut x = 1;
            while g != 0 {
                g /= 10;
                x *= 10;
            }
            helper0[i] = x;
        }
        let mut helper1 = vec![0; 1 << n];
        for bits in 0..(1 << n) {
            let mut t = 1;
            for i in 0..n {
                if bits & (1 << i) != 0 {
                    t = t * helper0[i] % k;
                }
            }
            helper1[bits] = t;
        }
        let k = k as usize;

        let mut dp = vec![vec![Vec::<i32>::new(); k]; 1 << n];
        for i in 0..n {
            let t = (nums[i] % k as i32) as usize;
            dp[1 << i][t].push(nums[i]);
        }

        for bits in 1..(1 << n) {
            for i in 0..n {
                if bits & (1 << i) != 0 {
                    continue;
                }
                let next_bits = bits | (1 << i);
                let x = helper1[bits];
                for j in 0..k {
                    if dp[bits][j].is_empty() {
                        continue;
                    }
                    let t = ((nums[i] * x + j as i32) % k as i32) as usize;
                    let mut v = dp[bits][j].clone();
                    v.push(nums[i]);
                    if dp[next_bits][t].is_empty() {
                        dp[next_bits][t] = v;
                    } else {
                        if dp[next_bits][t].iter().rev().cmp(v.iter().rev())
                            == std::cmp::Ordering::Greater
                        {
                            dp[next_bits][t] = v;
                        }
                    }
                }
            }
        }
        let mut ans = dp[(1 << n) - 1][0].clone();
        ans.reverse();
        ans
    }
}
