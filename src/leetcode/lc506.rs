#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::cmp::Reverse;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score = score
            .into_iter()
            .enumerate()
            .map(|(idx, s)| (Reverse(s), idx))
            .collect::<Vec<_>>();
        score.sort_unstable();
        let medals = ["Gold Medal", "Silver Medal", "Bronze Medal"];
        let mut ans = vec![String::new(); score.len()];
        for (p, (_, i)) in score.into_iter().enumerate() {
            if p < 3 {
                ans[i] = medals[p].to_string();
            } else {
                ans[i] = (p + 1).to_string();
            }
        }
        ans
    }
}
