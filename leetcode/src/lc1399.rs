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
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = HashMap::new();
        for mut i in 1..=n {
            let mut k = 0;
            while i != 0 {
                k += i % 10;
                i /= 10;
            }
            *map.entry(k).or_insert(0) += 1;
        }
        let mut maxi = 0;
        let mut ans = 0;
        for (_, v) in map.into_iter() {
            if v > maxi {
                ans = 1;
                maxi = v;
            } else if v == maxi {
                ans += 1;
            }
        }
        ans
    }
}
