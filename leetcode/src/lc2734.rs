#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "aa".to_owned();
        let ans = "az".to_owned();
        assert_eq!(Solution::smallest_string(s), ans);
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

struct Solution {}

impl Solution {
    pub fn smallest_string(s: String) -> String {
        let mut s = s.into_bytes().into_iter().map(|e| e - b'a').collect::<Vec<_>>();
        let n = s.len();

        fn to_ans(s: Vec<u8>) -> String {
            let s = s.into_iter().map(|e| e + b'a').collect::<Vec<_>>();
            String::from_utf8(s).unwrap()
        }

        let a = s.iter().position(|e| *e != 0);
        if a.is_none() {
            s[n - 1] = 25;
            return to_ans(s);
        }
        let a = a.unwrap();
        for i in a..n {
            if s[i] == 0 {
                break;
            }
            s[i] = (s[i] + 26 - 1) % 26
        }
        to_ans(s)
    }
}
