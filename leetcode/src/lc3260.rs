#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 3;
        let k = 5;
        let ans = "595".to_owned();
        assert_eq!(Solution::largest_palindrome(n, k), ans);
    }

    #[test]
    fn test1() {
        let n = 5;
        let k = 6;
        let ans = "89898".to_owned();
        assert_eq!(Solution::largest_palindrome(n, k), ans);
    }

    #[test]
    fn test2() {
        let n = 3;
        let k = 7;
        let ans = "959".to_owned();
        assert_eq!(Solution::largest_palindrome(n, k), ans);
    }

    #[test]
    fn test3() {
        let n = 4;
        let k = 7;
        let ans = "9779".to_owned();
        assert_eq!(Solution::largest_palindrome(n, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn largest_palindrome(n: i32, k: i32) -> String {
        let mut ans = vec![0; n as usize];
        let mut remain10 = vec![0; n as usize];
        remain10[0] = 1;
        for i in 1..remain10.len() {
            remain10[i] = remain10[i - 1] * 10 % k;
        }
        let mut vis = vec![vec![false; k as usize]; n as usize];
        Self::dfs(0, 0, k, &remain10, &mut vis, &mut ans);
        String::from_utf8(ans).unwrap()
    }

    fn dfs(
        idx: usize,
        remain: i32,
        k: i32,
        remain10: &Vec<i32>,
        vis: &mut Vec<Vec<bool>>,
        ans: &mut Vec<u8>,
    ) -> bool {
        let n = ans.len();
        if idx >= (n + 1) / 2 {
            return remain == 0;
        }
        if vis[idx][remain as usize] {
            return false;
        }
        for i in (0..10).rev() {
            let mut r = remain10[idx] * i;
            if n - 1 - idx != idx {
                r += remain10[n - 1 - idx] * i;
            }
            r = (remain + r) % k;
            if Self::dfs(idx + 1, r, k, remain10, vis, ans) {
                ans[idx] = b'0' + i as u8;
                ans[n - 1 - idx] = b'0' + i as u8;
                return true;
            }
        }
        vis[idx][remain as usize] = true;
        false
    }
}
