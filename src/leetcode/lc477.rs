struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
		const HIGHEST_BIT: usize = 31;
        let mut ones = vec![0; HIGHEST_BIT];
		for &num in nums.iter() {
            for k in 0..HIGHEST_BIT {
				if (num & (1 << k)) != 0 {
                    ones[k] += 1;
				}
			}
		}
		ones.into_iter().map(|x| x * (nums.len() as i32 - x)).sum()
    }
}