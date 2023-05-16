#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut ans = vec![0; derived.len()];
        for i in 0..derived.len() - 1 {
            ans[i + 1] = derived[i] ^ ans[i];
        }
        ans[0] ^ ans[ans.len() - 1] == derived[derived.len() - 1]
    }
}