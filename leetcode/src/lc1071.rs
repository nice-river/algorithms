#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s1 = "LEET".to_owned();
        let s2 = "CODE".to_owned();
        assert_eq!(Solution::gcd_of_strings(s1, s2), "".to_owned());
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
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let s1 = str1.as_bytes();
        let s2 = str2.as_bytes();
        'outer: for i in (1..=s2.len()).rev() {
            if s1.len() % i != 0 || s2.len() % i != 0 {
                continue;
            }
            if &s1[0..i] != &s2[0..i] {
                continue 'outer;
            }
            for k in (0..s1.len()).step_by(i).skip(1) {
                if &s1[0..i] != &s1[k..k + i] {
                    continue 'outer;
                }
            }
            for k in (0..s2.len()).step_by(i).skip(1) {
                if &s2[0..i] != &s2[k..k + i] {
                    continue 'outer;
                }
            }
            return std::str::from_utf8(&s1[0..i]).unwrap().to_owned();
        }
        "".to_owned()
    }
}
