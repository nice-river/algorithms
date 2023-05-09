#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "()())()";
        let ans = ["()()()", "(())()"];
        let ans = ans.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(Solution::remove_invalid_parentheses(s.to_string()), ans);
    }

    #[test]
    fn test1() {
        let s = ")(";
        let ans = [""];
        let ans = ans.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(Solution::remove_invalid_parentheses(s.to_string()), ans);
    }
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let (mut dl, mut dr) = (0, 0);
        for &ch in s {
            if ch == b'(' {
                dl += 1;
            } else if ch == b')' {
                if dl == 0 {
                    dr += 1;
                } else {
                    dl -= 1;
                }
            }
        }
        let mut ans = HashSet::new();
        Solution::dfs(0, s, dl, dr, &mut vec![], 0, &mut ans);
        ans.into_iter().collect()
    }

    fn dfs(
        idx: usize,
        s: &[u8],
        dl: i32,
        dr: i32,
        cur: &mut Vec<u8>,
        unpaired: i32,
        ans: &mut HashSet<String>,
    ) {
        if idx == s.len() {
            if dl == 0 && dr == 0 && unpaired == 0 {
                ans.insert(String::from_utf8(cur.clone()).unwrap());
            }
            return;
        }
        if s[idx] == b'(' {
            cur.push(s[idx]);
            Solution::dfs(idx + 1, s, dl, dr, cur, unpaired + 1, ans);
            cur.pop();
            if dl > 0 {
                Solution::dfs(idx + 1, s, dl - 1, dr, cur, unpaired, ans);
            }
        } else if s[idx] == b')' {
            if unpaired == 0 {
                if dr > 0 {
                    Solution::dfs(idx + 1, s, dl, dr - 1, cur, unpaired, ans);
                } else {
                    return;
                }
            } else {
                cur.push(s[idx]);
                Solution::dfs(idx + 1, s, dl, dr, cur, unpaired - 1, ans);
                cur.pop();
                if dr > 0 {
                    Solution::dfs(idx + 1, s, dl, dr - 1, cur, unpaired, ans);
                }
            }
        } else {
            cur.push(s[idx]);
            Solution::dfs(idx + 1, s, dl, dr, cur, unpaired, ans);
            cur.pop();
        }
    }
}
