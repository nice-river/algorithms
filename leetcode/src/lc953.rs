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

use crate::*;

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = HashMap::new();
        for (i, c) in order.bytes().enumerate() {
            map.insert(c, i);
        }
        for i in 1..words.len() {
            if Self::greater(&words[i - 1], &words[i], &map) {
                return false;
            }
        }
        true
    }

    fn greater(a: &str, b: &str, order: &HashMap<u8, usize>) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();
        for i in 0.. {
            match (a.get(i), b.get(i)) {
                (Some(a), Some(b)) => {
                    let a = order.get(a).unwrap();
                    let b = order.get(b).unwrap();
                    if a < b {
                        return false;
                    } else if a > b {
                        return true;
                    }
                }
                (Some(a), None) => {
                    return true;
                }
                (None, Some(b)) => {
                    return false;
                }
                (None, None) => break,
            }
        }
        false
    }
}
