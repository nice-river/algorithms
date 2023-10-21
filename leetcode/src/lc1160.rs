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
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        fn to_map(s: &String) -> HashMap<char, i32> {
            let mut map = HashMap::new();
            for ch in s.chars() {
                *map.entry(ch).or_insert(0) += 1;
            }
            map
        }
        let all = to_map(&chars);
        let mut ans = 0;
        'outer: for word in &words {
            let w = to_map(word);
            for (k, v) in w.into_iter() {
                if let Some(&t) = all.get(&k) {
                    if t < v {
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }
            ans += word.len() as i32;
        }
        ans
    }
}
