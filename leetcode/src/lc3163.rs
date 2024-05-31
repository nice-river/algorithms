#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let word = "aaaaaaaaaaaaaabb".to_owned();
        let ans = "9a5a2b".to_owned();
        assert_eq!(Solution::compressed_string(word), ans);
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
    pub fn compressed_string(word: String) -> String {
        let mut stk = vec![];
        let mut ans = vec![];
        for &ch in word.as_bytes() {
            match stk.last() {
                Some(&x) if x == ch => {
                    if stk.len() == 9 {
                        ans.push(b'9');
                        ans.push(x);
                        stk.clear();
                    }
                }
                Some(&x) => {
                    ans.push(b'0' + stk.len() as u8);
                    ans.push(x);
                    stk.clear();
                }
                None => {}
            }
            stk.push(ch);
        }
        if let Some(&x) = stk.last() {
            ans.push(b'0' + stk.len() as u8);
            ans.push(x);
        }
        String::from_utf8(ans).unwrap()
    }
}
