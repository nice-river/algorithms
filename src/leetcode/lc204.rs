struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn count_primes(n: i32) -> i32 {
		if n <= 1 {
			return 0;
		}
		let n = n as usize;
		let mut arr = vec![false; n];
		for i in 2..n {
			if !arr[i] {
				let mut k = 2;
				while k * i < n {
					arr[k * i] = true;
					k += 1;
				}
			}
		}
		arr.iter().filter(|&&v| !v).count() as i32 - 2
	}
}