use rand::seq::SliceRandom;

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
    pub fn mask_pii(s: String) -> String {
        let s = s.split('@').into_iter().collect::<Vec<_>>();
        if s.len() == 2 {
            let a = s[0].to_ascii_lowercase();
            let b = s[1].to_ascii_lowercase();
            let a = a.as_bytes();
            let mut ans = vec![];
            ans.push(a[0]);
            for _ in 0..5 {
                ans.push(b'*');
            }
            ans.push(*a.last().unwrap());
            ans.push(b'@');
            for c in b.as_bytes() {
                ans.push(*c);
            }
            String::from_utf8(ans).unwrap()
        } else {
            let mut digits = vec![];
            for &c in s[0].as_bytes() {
                if b'0' <= c && c <= b'9' {
                    digits.push(c);
                }
            }
            let mut ans = vec![];
            let mut i = 0;
            if digits.len() > 10 {
                ans.push(b'+');
                for _ in 0..digits.len() - 10 {
                    ans.push(b'*');
                    i += 1;
                }
                ans.push(b'-');
            }
            let mut c = 0;
            while i < digits.len() {
                c += 1;
                if c <= 6 {
                    ans.push(b'*');
                } else {
                    ans.push(digits[i]);
                }
                if c == 3 || c == 6 {
                    ans.push(b'-');
                }
                i += 1;
            }
            String::from_utf8(ans).unwrap()
        }
    }
}
