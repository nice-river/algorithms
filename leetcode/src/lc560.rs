#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut s = 0;
        let mut map = HashMap::new();
        let mut ans = 0;
        map.insert(0, 1);
        for num in nums {
            s += num;
            ans += *map.get(&(s - k)).unwrap_or(&0);
            *map.entry(s).or_insert(0) += 1;
        }
        ans
    }
}
