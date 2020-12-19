struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut matrix = vec![];
		for i in 0..6 {
			matrix.push((0..6).map(|n| i * 6 + n).collect::<Vec<i32>>());
		}
		for i in 0..6 {
			for j in 0..6 {
				print!("{} ", matrix[i][j]);
			}
			println!();
		}
		println!();
		Solution::rotate(&mut matrix);
		for i in 0..6 {
			for j in 0..6 {
				print!("{} ", matrix[i][j]);
			}
			println!();
		}
	}
}



impl Solution {
	pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
		let mut s = 0;
		let mut e = matrix.len();
		while s + 1 < e {
			Solution::rotate_layer(s, e, matrix);
			s += 1;
			e -= 1;
		}
	}

	fn rotate_layer(s: usize, e: usize, matrix: &mut Vec<Vec<i32>>) {
		for k in s..e-1 {
			let i = k - s;
			let t = matrix[s][s+i];
			matrix[s][s+i] = matrix[s+i][e-1];
			matrix[s+i][e-1] = t;

			let t = matrix[s][s+i];
			matrix[s][s+i] = matrix[e-1][e-1-i];
			matrix[e-1][e-1-i] = t;

			let t = matrix[s][s+i];
			matrix[s][s+i] = matrix[e-1-i][s];
			matrix[e-1-i][s] = t;
		}
	}
}