#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let dictionary = vec!["apple", "monkey", "plea", "ale"];
        let dictionary = dictionary.into_iter().map(|s| s.to_string()).collect();
        let s = String::from("abpcplea");
        let ans = String::from("apple");
        assert_eq!(Solution::find_longest_word(s, dictionary), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut dictionary = dictionary.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        dictionary.sort_unstable_by_key(|&s| (-(s.len() as i32), s));
        let s = s.as_bytes();
        'outer: for word in dictionary {
            let mut i = 0;
            for &ch in word {
                while i < s.len() && s[i] != ch {
                    i += 1;
                }
                if i == s.len() {
                    continue 'outer;
                }
                i += 1;
            }
            return String::from_utf8(word.to_vec()).unwrap();
        }
        "".to_string()
    }
}
