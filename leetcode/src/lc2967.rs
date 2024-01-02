#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![301, 309, 312, 322];
        let ans = 26;
        assert_eq!(Solution::minimum_cost(nums), ans);
    }

    #[test]
    fn test1() {
        let mut nums = vec![710, 800, 407, 522, 536, 485, 400, 823, 438, 866];
        nums.sort();
        let ans = 1483;
        assert_eq!(Solution::minimum_cost(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1000000000; 500];
        let ans = 500;
        assert_eq!(Solution::minimum_cost(nums), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![202, 206, 211, 214, 224, 226];
        let ans = 45;
        assert_eq!(Solution::minimum_cost(nums), ans);
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
    pub fn minimum_cost(mut nums: Vec<i32>) -> i64 {
        let mut ans = i64::MAX;
        nums.sort();
        let mut s = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            s[i + 1] = s[i] + nums[i] as i64;
        }
        let ps = Self::generate_p();
        let n = nums.len() as i64;
        if let Some((_, b)) = Self::find(1, nums[0] as i64, &ps) {
            ans = s[nums.len()] - b * n;
        }
        if let Some((a, _)) = Self::find(nums[nums.len() - 1] as i64, 1_000_000_000, &ps) {
            ans = ans.min(a * n - s[nums.len()]);
        }
        for i in 1..n {
            if let Some((a, b)) =
                Self::find(nums[i as usize - 1] as i64, nums[i as usize] as i64, &ps)
            {
                ans = ans.min(a * i - s[i as usize] + s[n as usize] - s[i as usize] - (n - i) * a);
                ans = ans.min(b * i - s[i as usize] + s[n as usize] - s[i as usize] - (n - i) * b);
            }
        }
        ans
    }

    fn generate_p() -> Vec<i64> {
        let mut ret = vec![];
        let mut x = Vec::with_capacity(20);
        for mut i in 1..10000 {
            if i % 10 == 0 {
                continue;
            }
            x.clear();
            while i != 0 {
                x.push(i % 10);
                i /= 10;
            }
            let mut t = x.clone();
            t.reverse();
            let mut g = 0;
            for &v in x.iter().chain(t.iter()) {
                g = g * 10 + v;
            }
            ret.push(g);
            g = 0;
            for &v in x.iter().chain(t.iter().skip(1)) {
                g = g * 10 + v;
            }
            ret.push(g);
            while x.len() + t.len() < 9 {
                x.push(0);
                g = 0;
                for &v in x.iter().chain(t.iter()) {
                    g = g * 10 + v;
                }
                ret.push(g);
            }
        }
        for mut i in 10000..100000 {
            if i % 10 == 0 {
                continue;
            }
            x.clear();
            while i != 0 {
                x.push(i % 10);
                i /= 10;
            }
            let mut t = x.clone();
            t.reverse();
            let mut g = 0;
            for &v in x.iter().chain(t.iter().skip(1)) {
                g = g * 10 + v;
            }
            ret.push(g);
            while x.len() + t.len() < 9 {
                x.push(0);
                g = 0;
                for &v in x.iter().chain(t.iter()) {
                    g = g * 10 + v;
                }
                ret.push(g);
            }
        }
        ret.sort();
        ret
    }

    fn find(a: i64, b: i64, arr: &Vec<i64>) -> Option<(i64, i64)> {
        let mut p = (-1, -1);
        let mut l = 0;
        let mut r = arr.len();
        while l < r {
            let m = (l + r) / 2;
            if arr[m] < a {
                l = m + 1;
            } else if arr[m] >= a {
                p.0 = arr[m];
                r = m;
            }
        }
        l = 0;
        r = arr.len();
        while l < r {
            let m = (l + r) / 2;
            if arr[m] <= b {
                p.1 = arr[m];
                l = m + 1;
            } else if arr[m] > b {
                r = m;
            }
        }

        if p.0 == -1 || p.0 > b {
            None
        } else {
            Some(p)
        }
    }
}
