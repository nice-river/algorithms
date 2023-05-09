#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        let lines = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];
        for (i, line) in lines.iter().enumerate() {
            for &ch in line.as_bytes() {
                map.insert(ch, i);
            }
        }
        let mut ans = vec![];
        for word in words {
            if word
                .as_bytes()
                .windows(2)
                .all(|w| map.get(&w[0].to_ascii_lowercase()) == map.get(&w[1].to_ascii_lowercase()))
            {
                ans.push(word);
            }
        }
        ans
    }
}
