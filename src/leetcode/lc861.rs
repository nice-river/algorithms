struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
		let mut ans = 0;
		let n = a.len();
		let m = a[0].len();

		let mut row_flip = vec![false; n];
		let mut col_flip = vec![false; m];

		for i in 0..n {
			if a[i][0] == 0 {
				row_flip[i] = true;
			}
		}

		for j in 1..m {
			let (mut x, mut y) = (0, 0);
			for i in 0..n {
				if (row_flip[i] as i32 ^ a[i][j]) == 1 {
					x += 1;
				} else {
					y += 1;
				}
			}
			if x < y {
				col_flip[j] = true;
			}
		}

		for i in 0..n {
			let mut r = 0;
			for j in 0..m {
				if row_flip[i] ^ col_flip[j] {
					r = r * 2 + (1 - a[i][j]);
				} else {
					r = r * 2 + a[i][j];
				}
			}
			ans += r;
		}

		ans
	}
}