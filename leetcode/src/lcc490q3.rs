#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn maximum_xor(s: String, t: String) -> String {
        let mut cnt = vec![0; 2];
        for ch in t.chars() {
            match ch {
                '0' => cnt[0] += 1,
                '1' => cnt[1] += 1,
                _ => unreachable!(),
            }
        }
        let mut ans = String::new();
        for ch in s.chars() {
            match ch {
                '0' => {
                    if cnt[1] > 0 {
                        ans.push('1');
                        cnt[1] -= 1;
                    } else {
                        ans.push('0');
                        cnt[0] -= 1;
                    }
                }
                '1' => {
                    if cnt[0] > 0 {
                        ans.push('1');
                        cnt[0] -= 1;
                    } else {
                        ans.push('0');
                        cnt[1] -= 1;
                    }
                }
                _ => unreachable!(),
            }
        }
        ans
    }
}
