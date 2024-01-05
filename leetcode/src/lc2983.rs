#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "abcabc".to_owned();
        let queries = to_vec2d([[1,1,3,5],[0,2,5,5]]);
        let ans = vec![true, true];
        assert_eq!(Solution::can_make_palindrome_queries(s, queries), ans);
    }

    #[test]
    fn test1() {
        let s = "bccbcc".to_owned();
        let queries = to_vec2d([[1,2,4,5],[0,2,3,5]]);
        let ans = vec![false, true];
        assert_eq!(Solution::can_make_palindrome_queries(s, queries), ans);
    }

    #[test]
    fn test2() {
        let s = "bbccbb".to_owned();
        let queries = to_vec2d([[0, 1, 4, 5]]);
        let ans = vec![true];
        assert_eq!(Solution::can_make_palindrome_queries(s, queries), ans);
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

const N: usize = 26;

impl Solution {
    pub fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let s = s.into_bytes();
        let n = s.len();
        let x = s.iter().take(n / 2).copied().collect::<Vec<_>>();
        let y = s.iter().skip(n / 2).rev().copied().collect::<Vec<_>>();

        let n = x.len();
        let mut same = vec![];
        let mut i = 0;
        while i < x.len() {
            if x[i] != y[i] {
                i += 1;
            } else {
                let mut j = i + 1;
                while j < x.len() && x[j] == y[j] {
                    j += 1;
                }
                same.push((i, j));
                i = j;
            }
        }
        let mut p = vec![vec![0; N]; x.len() + 1];
        let mut q = vec![vec![0; N]; x.len() + 1];
        let mut cover = vec![true; x.len()];
        for i in 0..x.len() {
            let mut f = true;
            for c in 0..N {
                p[i + 1][c] = p[i][c];
                if c as u8 == x[i] - b'a' {
                    p[i + 1][c] += 1;
                }
                q[i + 1][c] = q[i][c];
                if c as u8 == y[i] - b'a' {
                    q[i + 1][c] += 1;
                }
                if p[i + 1][c] < q[i + 1][c] {
                    f = false;
                }
            }
            cover[i] = f;
        }
        let mut ans = vec![false; queries.len()];
        if !cover[x.len() - 1] {
            return ans;

        }
        for (i, query) in queries.into_iter().enumerate() {
            let a = query[0] as usize;
            let b = query[1] as usize + 1;
            let c = n - 1 - (query[3] as usize - n);
            let d = n - 1 - (query[2] as usize - n) + 1;
            if a < c {
                if Self::check(a, b, c, d, &same, &p, &q, &cover) {
                    ans[i] = true;
                }
            } else if Self::check(c, d, a, b, &same, &q, &p, &cover) {
                ans[i] = true;
            }
        }
        ans
    }

    fn check(a: usize, b: usize, c: usize, d: usize, same: &Vec<(usize, usize)>, p: &Vec<Vec<i32>>, q: &Vec<Vec<i32>>, cover: &Vec<bool>) -> bool {
        if c >= b {
            if Self::is_same(0, a, same) && cover[b - 1] && Self::is_same(b, c, &same) && cover[d - 1] && Self::is_same(d, cover.len(), &same) {
                return true;
            }
            return false;
        } else {
            if d <= b {
                if Self::is_same(0, a, same) && cover[b - 1] && Self::is_same(b, cover.len(), same) {
                    return true;
                }
                return false;
            } else {
                if !Self::is_same(0, a, same) || !Self::is_same(d, cover.len(), same) {
                    return false;
                }
                for i in 0..N {
                    if p[b][i] - p[a][i] < q[c][i] - q[a][i] {
                        return false;
                    }
                }
            }
            return true;
        }
    }

    fn is_same(a: usize, b: usize, same: &Vec<(usize, usize)>) -> bool {
        if a == b {
            return true;
        }
        if same.is_empty() {
            return false;
        }
        let mut l = 0;
        let mut r = same.len();
        while l < r {
            let m = (l + r) / 2;
            if same[m].0 <= a {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if r == 0 {
            false
        } else {
            same[r - 1].1 >= b
        }
    }
}