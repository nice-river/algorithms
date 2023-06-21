#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![773,613,11,8,103];
        let k = 40;
        assert_eq!(Solution::subarray_lcm(nums, k), 0);
    }

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
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut t = 1;
            for j in i..nums.len() {
                let g = Self::gcd(t, nums[j]);
                if t / g * nums[j] == k {
                    ans += 1;
                }
                t = t / g * nums[j];
                if t > k {
                    break;
                }
            }
        }
        ans as i32
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
