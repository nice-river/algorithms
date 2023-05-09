use std::process::id;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "aab".to_string();
        let goal = "aab".to_string();
        assert!(Solution::buddy_strings(s, goal));
    }
}

struct Solution {}

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let s = s.as_bytes();
        let goal = goal.as_bytes();
        if s.len() != goal.len() {
            return false;
        }
        let mut idxes = vec![];
        for i in 0..s.len() {
            if s[i] != goal[i] {
                idxes.push(i);
            }
        }
        if idxes.len() == 0 {
            let mut ch_cnt = vec![0; 26];
            for &c in s {
                ch_cnt[(c - b'a') as usize] += 1;
            }
            ch_cnt.into_iter().any(|c| c > 1)
        } else if idxes.len() == 2 {
            s[idxes[0]] == goal[idxes[1]] && s[idxes[1]] == goal[idxes[0]]
        } else {
            false
        }
    }
}
