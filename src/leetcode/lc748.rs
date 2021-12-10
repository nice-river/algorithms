#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let cnt = Self::count_char(&license_plate);
        let mut ans: Option<String> = None;
        for word in words.into_iter().rev() {
            let word_cnt = Self::count_char(&word);
            if word_cnt.iter().zip(cnt.iter()).all(|(a, b)| a >= b) {
                if let Some(ref s) = ans {
                    if s.len() >= word.len() {
                        ans = Some(word.clone());
                    }
                } else {
                    ans = Some(word.clone());
                }
            }
        }
        ans.unwrap_or(String::new())
    }

    fn count_char(s: &str) -> Vec<i32> {
        let mut cnt = vec![0; 26];
        for &c in s.as_bytes() {
            if c.is_ascii_alphabetic() {
                cnt[(c.to_ascii_lowercase() - b'a') as usize] += 1;
            }
        }
        cnt
    }
}
