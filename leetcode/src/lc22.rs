#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = vec![];
        let mut cur = vec!["("];
        Self::dfs(&mut cur, n - 1, 1, &mut ans);
        ans
    }

    fn dfs(cur: &mut Vec<&'static str>, n: i32, unmatch: i32, ans: &mut Vec<String>) {
        if n == 0 && unmatch == 0 {
            ans.push(cur.join(""));
            return;
        }
        if n > 0 {
            cur.push("(");
            Self::dfs(cur, n - 1, unmatch + 1, ans);
            cur.pop();
        }
        if unmatch > 0 {
            cur.push(")");
            Self::dfs(cur, n, unmatch - 1, ans);
            cur.pop();
        }
    }
}
