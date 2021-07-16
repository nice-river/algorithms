struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.into_iter().filter(|&x| x == target).count() as i32
    }
}