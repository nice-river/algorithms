#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

struct Solution {}

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}
