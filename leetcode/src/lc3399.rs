#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "000001";
        let num_ops = 1;
        let ans = 2;
        assert_eq!(Solution::min_length(s.to_owned(), num_ops), ans);
    }

    #[test]
    fn test1() {
        let s = "0110";
        let num_ops = 1;
        let ans = 2;
        assert_eq!(Solution::min_length(s.to_owned(), num_ops), ans);
    }

    #[test]
    fn test2() {
        let s = "01101";
        let num_ops = 2;
        let ans = 1;
        assert_eq!(Solution::min_length(s.to_owned(), num_ops), ans);
    }

    #[test]
    fn test3() {
        let s = "1011111000111111111111000100110101010111001101011010010111111000000110000101000101101011100001001011";
        let num_ops = 53;
        let ans = 1;
        assert_eq!(Solution::min_length(s.to_owned(), num_ops), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        let s = s.as_bytes();
        let mut s = s.iter().map(|b| *b - b'0').collect::<Vec<_>>();
        if Self::helper1(s.clone(), num_ops) {
            return 1;
        }
        if num_ops > 0 {
            let mut ss = s.clone();
            ss[0] ^= 1;
            if Self::helper1(ss, num_ops - 1) {
                return 1;
            }
        }
        let a = Self::helper0(&s, num_ops);
        s.reverse();
        a.min(Self::helper0(&s, num_ops))
    }

    fn helper1(mut s: Vec<u8>, mut num_ops: i32) -> bool {
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                if num_ops > 0 {
                    num_ops -= 1;
                    s[i] ^= 1;
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn helper0(s: &[u8], num_ops: i32) -> i32 {
        let mut left = 2;
        let mut right = s.len();
        while left < right {
            let m = (left + right) / 2;
            let mut c = 1;
            let mut ops = 0;
            let mut checked = true;
            let mut is_rev = false;

            for i in 1..s.len() {
                let pre = if is_rev { s[i - 1] ^ 1 } else { s[i - 1] };
                if s[i] != pre {
                    c = 1;
                    is_rev = false;
                } else {
                    c += 1;
                    if c > m {
                        if ops + 1 > num_ops {
                            checked = false;
                            break;
                        }
                        ops += 1;
                        if i + 1 < s.len() {
                            is_rev = s[i + 1] == s[i];
                        }
                        c = 1;
                    } else {
                        is_rev = false;
                    }
                }
            }
            if checked {
                right = m;
            } else {
                left = m + 1;
            }
        }
        right as i32
    }
}
