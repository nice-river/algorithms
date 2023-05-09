#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split(' ')
            .into_iter()
            .take(k as usize)
            .collect::<Vec<_>>()
            .join(" ")
    }
}
