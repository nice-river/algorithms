struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![3,4,5,2,1,7,3,4,7];
		let k = 3;
		let ans = 3;
		assert_eq!(Solution::min_changes(nums, k), ans);
	}

	#[test]
	fn test1() {
		let nums = vec![3,4,5,2,1,7,3,4,7];
		let k = 9;
		let ans = 1;
		assert_eq!(Solution::min_changes(nums, k), ans);
	}
}

use std::collections::HashMap;

impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
		const MAX_VAL: usize = 1 << 10;
        let k = k as usize;
		let mut maps = vec![HashMap::new(); k];
		let dp = &mut vec![vec![-1; MAX_VAL]; k + 1];
		dp[0][0] = 0;
		for i in 0..k {
			let mut j = i;
            let mut tot = 0;
            while j < nums.len() {
                *maps[i].entry(nums[j]).or_insert(0) += 1;
				j += k;
				tot += 1;
			}
			for (&num, &cnt) in maps[i].iter() {
				for j in 0..MAX_VAL {
					if dp[i][j] != -1 {
						let p = j ^ num as usize;
						dp[i + 1][p] = dp[i + 1][p].max(dp[i][j] + cnt);
					}
				}
			}
		}
		let mut arr = Vec::with_capacity(k);
        for map in maps.iter() {
			let t = map.iter().map(|(k, v)| v).max().unwrap();
			arr.push(*t);
		}
		arr.sort_unstable();
		let ans: i32 = nums.len() as i32 - arr.into_iter().skip(1).sum::<i32>();
        ans.min(nums.len() as i32 - dp[k][0])
    }
}