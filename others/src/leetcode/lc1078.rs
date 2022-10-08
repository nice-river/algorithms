#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let text = text.split(" ").collect::<Vec<_>>();
        let mut ans = vec![];
        for s in text.windows(3) {
            if s[0] == &first && s[1] == &second {
                ans.push(s[2].to_string());
            }
        }
        ans
    }
}
