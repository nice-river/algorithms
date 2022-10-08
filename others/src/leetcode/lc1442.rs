struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
		let mut ans = 0;
		for i in 0..arr.len() {
            let mut x = arr[i];
			for j in i+1..arr.len() {
                x ^= arr[j];
				if x == 0 {
                    ans += j - i;
				}
			}
		}
		ans as i32
    }
}