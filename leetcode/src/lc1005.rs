#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums: Vec<i32> = Vec::new();
        assert_eq!(nums.into_iter().sum::<i32>(), 0);
    }
}

struct Solution {}

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_unstable();
        for num in nums.iter_mut() {
            if *num >= 0 {
                break;
            } else {
                *num = -*num;
                k -= 1;
                if k == 0 {
                    break;
                }
            }
        }
        if k % 2 == 1 {
            let min_idx = nums
                .iter()
                .enumerate()
                .min_by_key(|(idx, num)| *num)
                .map(|(idx, _)| idx)
                .unwrap();
            nums[min_idx] *= -1;
        }
        nums.into_iter().sum()
    }
}
