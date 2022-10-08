struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
		let mut ans = vec![0; encoded.len() + 1];
		let mut a = 0;
		let mut k = 0;
        for &e in encoded.iter() {
            k ^= e;
			a ^= k;
		}
		ans[0] = a ^ (1..=ans.len() as i32).fold(0, |a, b| a ^ b);
        for i in 1..ans.len() {
			ans[i] = encoded[i-1] ^ ans[i-1];
		}
		ans
    }
}