#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().count() as i32
    }
}
