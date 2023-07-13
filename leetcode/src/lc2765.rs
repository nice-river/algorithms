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
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let t = j - i;
                let t = if t % 2 == 0 { -1 } else { 1 };
                if nums[j] - nums[j - 1] == t {
                    ans = ans.max((j - i + 1) as i32);
                } else {
                    break;
                }
            }
        }
        ans
    }
}
