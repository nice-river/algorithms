#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![1, 0, 2];
        let ans = vec![0, 1, 2];
        assert_eq!(Solution::find_permutation(nums), ans);

        let nums = vec![0, 2, 1];
        let ans = vec![0, 2, 1];
        assert_eq!(Solution::find_permutation(nums), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![2, 1, 0, 3];
        let ans = vec![0, 1, 2, 3];
        assert_eq!(Solution::find_permutation(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1, 9, 2, 13, 3, 8, 4, 6, 11, 12, 5, 7, 10];
        let ans = vec![0, 1, 3, 5, 7, 6, 8, 12, 10, 13, 4, 11, 9, 2];
        assert_eq!(Solution::find_permutation(nums), ans);
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

use crate::*;

struct Solution {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Node(i32, usize);

impl Solution {
    pub fn find_permutation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut dp = vec![vec![None; n]; 1 << n]; // 第一个数字肯定是0，所以只需枚举最后一位即可
        dp[1][0] = Some(Node((0 - nums[0]).abs(), 0));
        let mut used = vec![];
        let mut not_used = vec![];
        let mut cnt = 0;
        for i in 1usize..(1 << n) {
            used.clear();
            not_used.clear();
            for b in 0..n {
                if (i & (1 << b)) != 0 {
                    used.push(b);
                } else {
                    not_used.push(b);
                }
            }
            for &y in &used {
                if let Some(Node(v, p)) = dp[i][y] {
                    for &z in &not_used {
                        cnt += 1;
                        let j = i | (1 << z);
                        let v = v + (y as i32 - nums[z]).abs() - (y as i32 - nums[0]).abs()
                            + (z as i32 - nums[0]).abs();
                        let nxt_node = Node(v, p * 15 + z);
                        if dp[j][z].is_none() || &nxt_node < dp[j][z].as_ref().unwrap() {
                            dp[j][z] = Some(nxt_node);
                        }
                    }
                }
            }
        }
        let mut ans = &Node(i32::MAX, usize::MAX);
        for y in 0..n {
            if let Some(v) = &dp[(1 << n) - 1][y] {
                ans = ans.min(v);
            }
        }
        let mut seq = ans.1;
        let mut ans = vec![]; 
        while seq != 0 {
            ans.push((seq % 15) as i32);
            seq /= 15;
        }
        if ans.len() != n {
            ans.push(0);
        }
        ans.reverse();
        ans
    }
}
