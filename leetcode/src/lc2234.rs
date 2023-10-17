#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let flowers = vec![1, 3, 1, 1];
        let new_flowers = 7;
        let target = 6;
        let full = 12;
        let partial = 1;
        let ans = 14;
        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            ans
        );
    }

    #[test]
    fn test1() {
        let flowers = vec![2, 4, 5, 3];
        let new_flowers = 10;
        let target = 5;
        let full = 2;
        let partial = 6;
        let ans = 30;
        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            ans
        );
    }

    #[test]
    fn test2() {
        let flowers = vec![8, 2];
        let new_flowers = 24;
        let target = 18;
        let full = 6;
        let partial = 3;
        let ans = 54;
        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            ans
        );
    }

    #[test]
    fn test3() {
        let flowers = vec![13];
        let new_flowers = 18;
        let target = 15;
        let full = 9;
        let partial = 2;
        let ans = 28;
        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            ans
        );
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
    pub fn maximum_beauty(
        flowers: Vec<i32>,
        mut new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        let mut flowers = flowers.into_iter().map(|e| e as i64).collect::<Vec<_>>();
        let mut ans = 0;
        let full = full as i64;
        let partial = partial as i64;
        let target = target as i64;
        flowers.sort();
        let n = flowers.len();
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + flowers[i];
        }
        let mut c = 0;
        for i in 1..=n {
            if flowers[n - i] >= target {
                c += 1;
            } else {
                break;
            }
        }
        if c == 0 {
            let t = Self::check(&flowers, &s, new_flowers);
            ans = t.min(target - 1) * partial;
            c += 1;
        }
        for i in c..=n {
            new_flowers -= (target - flowers[n - i]).max(0);
            if new_flowers < 0 {
                break;
            }
            let t = Self::check(&flowers[0..n - i], &s[0..=n - i], new_flowers).min(target - 1);
            ans = ans.max(t * partial + i as i64 * full);
        }
        ans
    }

    fn check(arr: &[i64], s: &[i64], fill: i64) -> i64 {
        if arr.len() == 0 {
            return 0;
        }
        let mut ret = 0;
        let mut l = arr[0];
        let mut r = arr[0] + fill + 1;
        while l <= r {
            let m = (l + r) / 2;
            match arr.binary_search(&m) {
                Ok(idx) | Err(idx) => {
                    if fill >= m * idx as i64 - s[idx] {
                        ret = m;
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }
            }
        }
        ret
    }
}
