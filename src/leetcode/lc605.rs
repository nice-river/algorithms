#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}


impl Solution {
	pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
		let mut k = 0;
		for i in 0..flowerbed.len() {
			if flowerbed[i] != 1 && (i == 0 || flowerbed[i-1] != 1) && (i == flowerbed.len() - 1 || flowerbed[i+1] != 1) {
				flowerbed[i] = 1;
				k += 1;
			}
		}
		k >= n
	}
}