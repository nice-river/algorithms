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
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; nums.len()];
        let mut s = 0;
        for i in 1..nums.len() {
            s += nums[i] - nums[0];
        }
        ans[0] = s;
        for i in 1..n {
            let c = nums[i] - nums[i - 1];
            s -= (n - i) as i32 * c;
            s += i as i32 * c;
            ans[i] = s;
        }
        ans
    }
}
