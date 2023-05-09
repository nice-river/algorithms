#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let richer = vec![vec![0, 1], vec![1, 2]];
        let quiet = vec![0, 1, 2];
        let ans = vec![0, 0, 0];
        assert_eq!(Solution::loud_and_rich(richer, quiet), ans);
    }
}

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut gph = vec![vec![]; n];
        let mut in_cnt = vec![0; n];
        for p in richer.iter() {
            gph[p[0] as usize].push(p[1] as usize);
            in_cnt[p[1] as usize] += 1;
        }
        let mut que = VecDeque::new();
        for (i, &c) in in_cnt.iter().enumerate() {
            if c == 0 {
                que.push_back(i);
            }
        }
        let mut ans = (0..n as i32).collect::<Vec<i32>>();
        while let Some(cur) = que.pop_front() {
            for &nxt in &gph[cur] {
                if quiet[ans[nxt] as usize] > quiet[ans[cur] as usize] {
                    ans[nxt] = ans[cur];
                }
                in_cnt[nxt] -= 1;
                if in_cnt[nxt] == 0 {
                    que.push_back(nxt);
                }
            }
        }
        ans
    }
}
