struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn check_inclusion(s1: String, s2: String) -> bool {
		if s2.len() < s1.len() {
			return false;
		}
		let s1 = s1.as_bytes();
		let s2 = s2.as_bytes();
		let mut cnt_s1 = [0; 30];
		let mut cnt_s2 = [0; 30];
		for &c in s1.iter() {
			cnt_s1[(c - b'a') as usize] += 1;
		}
		for i in 0..(s1.len() - 1){
			cnt_s2[(s2[i] - b'a') as usize] += 1;
		}
		for i in s1.len() - 1..s2.len() {
			cnt_s2[(s2[i] - b'a') as usize] += 1;
			for k in 0..30 {
				if cnt_s2[k] < cnt_s1[k] {
					break;
				}
			}
			if (0..30).into_iter().all(|idx| cnt_s2[idx] >= cnt_s1[idx]) {
				return true;
			}
			cnt_s2[(s2[i + 1 - s1.len()] - b'a') as usize] -= 1;
		}
		false
	}
}