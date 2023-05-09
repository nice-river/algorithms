#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::cmp::Reverse;

impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let mut s = num.to_string().into_bytes();
        if num >= 0 {
            s.sort_unstable();
            if s[0] == b'0' && s.len() > 1 {
                let mut j = 1;
                while j < s.len() && s[j] == b'0' {
                    j += 1;
                }
                s.swap(0, j);
            }
            String::from_utf8(s).unwrap().parse::<i64>().unwrap()
        } else {
            s.remove(0);
            s.sort_unstable_by_key(|k| Reverse(*k));
            -String::from_utf8(s).unwrap().parse::<i64>().unwrap()
        }
    }
}
