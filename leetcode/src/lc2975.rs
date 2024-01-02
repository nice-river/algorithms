#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let m = 3;
        let n = 9;
        let h_fences = vec![2];
        let v_fences = vec![8, 6, 5, 4];
        let ans = 4;
        assert_eq!(
            Solution::maximize_square_area(m, n, h_fences, v_fences),
            ans
        );
    }

    #[test]
    fn test1() {
        let m = 6;
        let n = 4;
        let h_fences = vec![3];
        let v_fences = vec![3, 2];
        let ans = 9;
        assert_eq!(
            Solution::maximize_square_area(m, n, h_fences, v_fences),
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

use std::collections::HashSet;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut ans = 0;
        let m = m as i64;
        let n = n as i64;
        let mut h_fences = h_fences.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut v_fences = v_fences.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        h_fences.sort();
        v_fences.sort();
        let mut h_gaps = HashSet::new();
        let mut v_gaps = HashSet::new();
        h_gaps.insert(m - 1);
        for i in 0..h_fences.len() {
            for j in i + 1..h_fences.len() {
                h_gaps.insert(h_fences[j] - h_fences[i]);
            }
            h_gaps.insert(h_fences[i] - 1);
            h_gaps.insert(m - h_fences[i]);
        }
        v_gaps.insert(n - 1);
        for i in 0..v_fences.len() {
            for j in i + 1..v_fences.len() {
                v_gaps.insert(v_fences[j] - v_fences[i]);
            }
            v_gaps.insert(v_fences[i] - 1);
            v_gaps.insert(n - v_fences[i]);
        }

        for g in h_gaps {
            if v_gaps.contains(&g) {
                ans = ans.max(g * g);
            }
        }

        if ans == 0 {
            -1
        } else {
            (ans % MOD) as i32
        }
    }
}
