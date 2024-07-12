#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let reward_values = vec![1, 6, 4, 3, 2];
        let ans = 11;
        assert_eq!(Solution::max_total_reward(reward_values), ans);
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

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let n = *reward_values.iter().max().unwrap() as usize;
        let mut num = vec![false; n + 1];
        for v in reward_values {
            num[v as usize] = true;
        }
        let mut arr = vec![false; n + 1];
        let mut ans = 0;
        arr[0] = true;
        for i in 0..=n {
            if !arr[i] {
                continue;
            }
            (i + 1..=n).for_each(|j| {
                if num[j] {
                    let t = i + j;
                    ans = ans.max(t);
                    if t <= n {
                        arr[t] = true;
                    }
                }
            });
        }
        ans as i32
    }
}
