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
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut a = 0;
        let mut b = 0;
        for (i, &num) in nums.iter().enumerate() {
            if num == 1 {
                a = i;
            } else if num == n as i32 {
                b = i;
            }
        }
        let mut ans = a + (n - 1) - b;
        if a > b {
            ans -= 1;
        }
        ans as i32
    }
}
