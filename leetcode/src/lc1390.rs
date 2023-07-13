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
    pub fn freq_alphabets(s: String) -> String {
        let mut i = 0;
        let s = s.as_bytes();
        let mut ans = vec![];
        while i < s.len() {
            if i + 2 < s.len() && s[i + 2] == b'#' {
                let n = std::str::from_utf8(&s[i..i + 2])
                    .unwrap()
                    .parse::<u8>()
                    .unwrap();
                ans.push(b'a' + n - 1);
                i += 3;
            } else {
                ans.push(b'a' + (s[i] - b'1'));
                i += 1;
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
