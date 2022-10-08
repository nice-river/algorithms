#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut ans = 1;
        let mut map = HashMap::new();
        for num in arr {
            if let Some(&x) = map.get(&(num - difference)) {
                ans = ans.max(x + 1);
                map.insert(num, x + 1);
            } else {
                map.insert(num, 1);
            }
        }
        ans
    }
}
