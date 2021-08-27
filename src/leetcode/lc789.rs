struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
		let x = Solution::dist(&vec![0, 0], &target);
		for ghost in ghosts.iter() {
			if Solution::dist(ghost, &target) <= x {
				return false;
			}
		}
		true
	}

	fn dist(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
		(a[0] - b[0]).abs() + (a[1] - b[1]).abs()
	}
}