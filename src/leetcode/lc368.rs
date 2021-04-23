struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, 2, 4, 8];
		assert_eq!(Solution::largest_divisible_subset(nums), vec![8, 4, 2, 1]);
	}
}


impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
		let mut max_cnt = 1;
        let mut max_idx = 0;
        let mut cnt = vec![1; nums.len()];
		let mut idx = vec![0; nums.len()];
		for i in 1..nums.len() {
			let mut cur_cnt = 0;
			let mut cur_idx = i;
			for j in 0..i {
				if nums[i] % nums[j] == 0 {
                    if cnt[j] > cur_cnt {
						cur_cnt = cnt[j];
						cur_idx = j;
					}
				}
			}
            cnt[i] = cur_cnt + 1;
            idx[i] = cur_idx;
			if cnt[i] > max_cnt {
				max_cnt = cnt[i];
				max_idx = i;
			}
		}
        let mut ans = vec![nums[max_idx]];
		while idx[max_idx] != max_idx {
			max_idx = idx[max_idx];
			ans.push(nums[max_idx]);
		}
        ans
    }
}