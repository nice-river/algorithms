#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let capital_cnt = word
            .as_bytes()
            .iter()
            .filter(|b| b.is_ascii_uppercase())
            .count();
        capital_cnt == word.len()
            || capital_cnt == 0
            || (capital_cnt == 1 && word.as_bytes()[0].is_ascii_uppercase())
    }
}
