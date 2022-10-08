struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
		let mut ans = 1;
		for i in 1..arr.len() {
			ans = std::cmp::min(ans + 1, arr[i] as i32);
		}
		ans
    }
}