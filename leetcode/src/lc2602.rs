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
    pub fn min_operations(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i64> {
        nums.sort();
        let mut s = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            s[i + 1] = s[i] + nums[i] as i64;
        }

        let mut ans = vec![];
        for q in queries {
            let mut l = 0;
            let mut r = nums.len();
            while l < r {
                let m = (l + r) / 2;
                if nums[m] >= q {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            let mut t = q as i64 * r as i64 - s[r];
            t += s[nums.len()] - s[r] - (nums.len() - r) as i64 * q as i64;
            ans.push(t);
        }

        ans
    }
}
