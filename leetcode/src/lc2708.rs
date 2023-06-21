use core::num;

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

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let mut pos = nums.iter().filter(|num| *num > &0).collect::<Vec<_>>();
        let mut neg = nums.iter().filter(|num| *num < &0).collect::<Vec<_>>();
        neg.sort_unstable();
        if pos.len() == 0 {
            if neg.len() <= 1 {
                return if nums.len() > neg.len() {
                    0
                } else {
                    *neg[0] as i64
                };
            }
        }
        let mut ans = 1;
        for &n in pos {
            ans *= n as i64;
        }
        let mut i = 0;
        while i < neg.len() {
            if i + 1 < neg.len() {
                ans *= *neg[i] as i64;
                ans *= *neg[i + 1] as i64;
            }
            i += 2;
        }
        ans
    }
}
