#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "0000".to_owned();
        let ans = 3;
        assert_eq!(Solution::num_ways(s), ans);
        let s = "100100010100110".to_owned();
        let ans = 12;
        assert_eq!(Solution::num_ways(s), ans);
        let s = "10101".to_owned();
        let ans = 4;
        assert_eq!(Solution::num_ways(s), ans);
        let s = "1001".to_owned();
        let ans = 0;
        assert_eq!(Solution::num_ways(s), ans);
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

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        const MODULE: i64 = 1_000_000_007;
        let s = s.as_bytes();
        let n = s.len();
        let mut accu = vec![0; n + 1];
        for i in 0..n {
            accu[i + 1] = accu[i] + if s[i] == b'0' { 0 } else { 1 };
        }
        let mut ans = 0i64;
        let all_one = accu[n];
        for i in 1..n {
            let rem = all_one - accu[i];
            if rem < accu[i] * 2 {
                break;
            }
            if rem != accu[i] * 2 {
                continue;
            }
            let mut a = 0;
            let mut l = i + 1;
            let mut r = n;
            while l <= r {
                let m = (l + r) / 2;
                if accu[m] == accu[i] * 2 {
                    a = m;
                    r = m - 1;
                } else if accu[m] < accu[i] * 2 {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
            if a == 0 || a == n {
                continue;
            }
            let mut b = 0;
            let mut l = a;
            let mut r = n;
            while l <= r {
                let m = (l + r) / 2;
                if accu[m] == accu[i] * 2 {
                    b = m;
                    l = m + 1;
                } else if accu[m] < accu[i] * 2 {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
            if b == 0 {
                continue;
            }
            ans += (b - a) as i64 + 1;
            if b == n {
                ans -= 1;
            }
            ans %= MODULE;
        }
        ans as i32
    }
}
