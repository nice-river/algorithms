#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut map = HashMap::new();
        let s = s.as_bytes();
        for i in 10..=s.len() {
            let dna = &s[i - 10..i];
            *map.entry(dna).or_insert(0) += 1;
        }
        let mut ans = vec![];
        for (dna, cnt) in map.into_iter() {
            if cnt > 1 {
                ans.push(String::from_utf8(dna.to_vec()).unwrap());
            }
        }
        ans
    }
}
