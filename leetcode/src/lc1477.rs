#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let arr = vec![2, 2, 1, 1, 1, 1];
        let target = 4;
        let ans = 6;
        assert_eq!(Solution::min_sum_of_lengths(arr, target), ans);
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

use std::collections::HashMap;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut cumu = vec![0; n + 1];
        let mut map: HashMap<i32, usize> = HashMap::new();
        map.insert(0, 0);
        for i in 1..=n {
            cumu[i] = cumu[i - 1] + arr[i - 1];
        }
        let mut s = 0;
        let mut intervals = vec![];
        for i in 1..=n {
            s += arr[i - 1];
            if let Some(p) = map.get(&(s - target)) {
                intervals.push((*p, i - 1));
            }
            map.insert(s, i);
        }
        if intervals.len() < 2 {
            return -1;
        }
        let mut a = vec![n + 1; n];
        let mut b = vec![n + 1; n];
        let mut p = 0;
        for i in 0..n {
            let mut t = if i == 0 { n + 1 } else { a[i - 1] };
            while p < intervals.len() && intervals[p].1 <= i {
                t = t.min(intervals[p].1 - intervals[p].0 + 1);
                p += 1;
            }
            a[i] = t;
        }
        let mut p = intervals.len() - 1;
        for i in (0..n).rev() {
            let mut t = if i == n - 1 { n + 1 } else { b[i + 1] };
            while p > 0 && intervals[p].0 >= i {
                t = t.min(intervals[p].1 - intervals[p].0 + 1);
                if let Some(q) = p.checked_sub(1) {
                    p = q;
                } else {
                    break;
                }
            }
            b[i] = t;
        }
        let mut ans = n + 1;
        for i in 0..n - 1 {
            if a[i] != n && b[i + 1] != n {
                ans = ans.min(a[i] + b[i + 1]);
            }
        }
        if ans == n + 1 {
            -1
        } else {
            ans as i32
        }
    }
}
