#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let strs = vec!["xc", "yb", "za"];
        let strs = strs.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        let ans = 0;
        assert_eq!(Solution::min_deletion_size(strs), ans);
    }

    #[test]
    fn test1() {
        let strs = vec!["ca", "bb", "ac"];
        let strs = strs.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        let ans = 1;
        assert_eq!(Solution::min_deletion_size(strs), ans);

        let strs = vec!["zyx", "wvu", "tsr"];
        let strs = strs.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        let ans = 3;
        assert_eq!(Solution::min_deletion_size(strs), ans);
    }

    #[test]
    fn test2() {
        let strs = vec!["vdy", "vei", "zvc", "zld"];
        let strs = strs.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        let ans = 2;
        assert_eq!(Solution::min_deletion_size(strs), ans);
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
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs = strs.into_iter().map(|s| s.into_bytes()).collect::<Vec<_>>();
        let mut segs = vec![(0usize, strs.len())];
        let mut cur_idx = 0;
        let n = strs[0].len();
        let mut ans = 0;
        while cur_idx != n {
            let back = segs.clone();
            let mut nxt_segs = vec![];
            let mut keep = true;
            for seg in segs {
                let (f, sub_segs) = Self::dfs(seg.0, seg.1, cur_idx, &strs);
                if !f {
                    keep = false;
                    break;
                } else {
                    nxt_segs.extend(sub_segs);
                }
            }
            if keep {
                segs = nxt_segs;
            } else {
                segs = back;
                ans += 1;
            }
            cur_idx += 1;
        }
        ans
    }

    fn dfs(a: usize, b: usize, idx: usize, strs: &Vec<Vec<u8>>) -> (bool, Vec<(usize, usize)>) {
        if a + 1 == b || idx == strs[0].len() {
            return (true, vec![(a, b)]);
        }
        let mut ordered = true;
        for i in a + 1..b {
            if strs[i][idx] < strs[i - 1][idx] {
                ordered = false;
                break;
            }
        }
        if !ordered {
            return (false, vec![]);
        } else {
            let mut ret = vec![];
            let mut i = a;
            while i < b {
                let mut j = i + 1;
                while j < b && strs[j][idx] == strs[i][idx] {
                    j += 1;
                }
                ret.push((i, j));
                i = j;
            }
            return (true, ret);
        }
    }
}
