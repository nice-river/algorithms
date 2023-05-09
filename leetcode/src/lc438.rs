#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "abab";
        let p = "ab";
        let ans = vec![0, 1, 2];
        assert_eq!(Solution::find_anagrams(s.to_string(), p.to_string()), ans);
    }
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.as_bytes();
        let p = p.as_bytes();
        if s.len() < p.len() {
            return vec![];
        }
        let mut p_cnt = vec![0; 256];
        let mut s_cnt = vec![0; 256];
        for &ch in p {
            p_cnt[ch as usize] += 1;
        }
        for &ch in &s[0..p.len()] {
            s_cnt[ch as usize] += 1;
        }
        let mut diffs = HashSet::new();
        for i in 0..256 {
            if p_cnt[i] != s_cnt[i] {
                diffs.insert(i);
            }
        }
        let mut ans = vec![];
        if diffs.is_empty() {
            ans.push(0);
        }
        for (i, _) in s.windows(p.len()).enumerate().skip(1) {
            let x = s[i - 1] as usize;
            let y = s[i - 1 + p.len()] as usize;
            s_cnt[x] -= 1;
            s_cnt[y] += 1;
            if s_cnt[x] != p_cnt[x] {
                diffs.insert(x);
            } else {
                diffs.remove(&x);
            }
            if s_cnt[y] != p_cnt[y] {
                diffs.insert(y);
            } else {
                diffs.remove(&y);
            }
            if diffs.is_empty() {
                ans.push(i as i32);
            }
        }
        ans
    }
}
