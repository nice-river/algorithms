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

use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut map = HashMap::new();
        for kv in knowledge.into_iter() {
            let mut kv = kv.into_iter();
            map.insert(kv.next().unwrap().into_bytes(), kv.next().unwrap());
        }

        let mut words = vec![];

        let s = s.as_bytes();
        let mut p = 0;
        let question = "?".to_owned();
        for i in 0..s.len() {
            if s[i] == b'(' {
                if i > p {
                    words.push(std::str::from_utf8(&s[p..i]).unwrap().to_string());
                }
                p = i + 1;
            } else if s[i] == b')' {
                words.push(map.get(&s[p..i]).unwrap_or_else(|| &question).clone());
                p = i + 1;
            }
        }
        if p < s.len() {
            words.push(std::str::from_utf8(&s[p..]).unwrap().to_string());
        }
        words.join("")
    }
}