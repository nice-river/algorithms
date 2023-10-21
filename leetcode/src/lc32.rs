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
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut stk = vec![];
        let mut intervals = vec![];
        for (i, &ch) in s.iter().enumerate() {
            if ch == b'(' {
                stk.push(i)
            } else if !stk.is_empty() {
                let p = stk.pop().unwrap();
                let mut x = p;
                let mut y = i;
                while let Some((a, b)) = intervals.pop() {
                    if b + 1 == x {
                        x = a;
                    } else if b + 1 < x {
                        intervals.push((a, b));
                        break;
                    }
                }
                ans = ans.max(y - x + 1);
                intervals.push((x, y));
            } else {
                intervals.clear();
            }
        }
        ans as i32
    }
}
