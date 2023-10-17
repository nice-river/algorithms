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

struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ans = vec![];
        let mut stk = vec![];
        for (i, ch) in s.char_indices() {
            if ch == '(' {
                stk.push(i);
            } else {
                if stk.len() == 1 {
                    if stk[0] + 1 != i {
                        ans.push(s[stk[0] + 1..i].to_string());
                    }
                }
                stk.pop();
            }
        }
        ans.join("")
    }
}