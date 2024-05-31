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

use crate::*;

struct Solution {}

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut s = vec![0; nums.len() + 1];
        for i in 1..nums.len() {
            let t = (nums[i] % 2) ^ (nums[i - 1] % 2);
            s[i] += s[i - 1] + t as usize;
        }

        let mut ans = Vec::with_capacity(queries.len());
        for query in queries {
            let a = query[0] as usize;
            let b = query[1] as usize;
            if s[b] - s[a] + 1 == (b - a) + 1 {
                ans.push(true);
            } else {
                ans.push(false);
            }
        }
        ans
    }
}
