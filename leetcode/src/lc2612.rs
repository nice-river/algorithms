#[cfg(test)]
mod tests {
    use crate::lc2612::Solution;

    #[test]
    fn test0() {
        let n = 5;
        let p = 0;
        let banned = vec![];
        let k = 4;
        let ans = vec![0, 3, 2, 1, 4];
        assert_eq!(Solution::min_reverse_operations(n, p, banned, k), ans);
    }

    #[test]
    fn test1() {
        let n = 18;
        let p = 1;
        let banned = vec![6, 17, 7, 5, 8, 12];
        let k = 14;
        let ans = vec![7, 0, 5, 2, 5, -1, -1, -1, -1, 4, 3, 4, -1, 6, 1, 6, 3, -1];
        assert_eq!(Solution::min_reverse_operations(n, p, banned, k), ans);
    }

    #[test]
    fn test2() {
        let n = 20;
        let p = 4;
        let banned = vec![8];
        let k = 14;
        let ans = vec![2, 3, 2, 3, 0, 3, 2, 3, -1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 4, 3];
        assert_eq!(Solution::min_reverse_operations(n, p, banned, k), ans);
    }

    #[test]
    fn test3() {
        let n = 125;
        let p = 11;
        let banned = vec![
            62, 30, 69, 114, 6, 89, 5, 65, 87, 111, 50, 72, 108, 84, 104, 32, 20, 115, 85, 55, 66,
            105, 101, 56, 88, 1, 113, 98, 15, 0, 80, 46, 97, 39, 12, 110, 45, 70, 21, 27, 106, 79,
            18, 59, 107, 37, 29, 7, 60, 52, 3, 41, 42, 73, 81, 49, 68, 124, 8, 13, 48, 82, 91, 47,
            102, 99, 75, 26, 10, 14, 36, 118, 2, 25, 93, 44, 96, 78,
        ];
        let k = 115;
        let ans = vec![
            -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, -1, 0, -1, -1, -1, -1, -1, 2, -1, 2, -1, -1, -1,
            2, -1, -1, -1, -1, -1, -1, -1, 2, -1, 4, -1, 4, -1, -1, -1, -1, -1, -1, -1, 4, -1, -1,
            -1, -1, -1, -1, -1, 4, -1, 6, -1, -1, -1, 6, -1, -1, -1, 6, -1, 5, -1, -1, -1, 5, -1,
            -1, -1, 5, -1, -1, -1, -1, -1, 5, -1, -1, -1, -1, -1, 3, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, 3, -1, -1, -1, -1, -1, -1, -1, 1, -1, -1, -1, -1, -1, 1, -1, -1, -1,
            -1, -1, -1, -1, 1, -1, 1, -1, 1, -1, 1, -1,
        ];
        assert_eq!(Solution::min_reverse_operations(n, p, banned, k), ans);
    }
}

struct Solution {}

use std::collections::BTreeSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        let mut dp = vec![i32::MAX; n as usize];
        for b in banned {
            dp[b as usize] = -1;
        }
        dp[p as usize] = 0;
        let mut que = VecDeque::new();
        que.push_back(p);
        let mut intervals = [BTreeSet::new(), BTreeSet::new()];
        for i in 0..n {
            intervals[if i % 2 == 0 { 0 } else { 1 }].insert(i);
        }
        while let Some(p) = que.pop_front() {
            let a = (p + k - 1).min(n - 1);
            let a = a - (p - (a - k + 1));
            let b = (p - k + 1).max(0);
            let b = b + (b + k - 1 - p);
            let l = a.min(b);
            let r = a.max(b);
            let w = if k % 2 == 0 {
                (p % 2) as usize ^ 1
            } else {
                (p % 2) as usize
            };
            let mut rm = vec![];
            for &i in intervals[w].range(l..=r) {
                if dp[i as usize] == i32::MAX {
                    dp[i as usize] = dp[p as usize] + 1;
                    que.push_back(i);
                }
                rm.push(i);
            }
            for k in rm {
                intervals[w].remove(&k);
            }
        }
        for v in dp.iter_mut() {
            if *v == i32::MAX {
                *v = -1;
            }
        }
        dp
    }
}
