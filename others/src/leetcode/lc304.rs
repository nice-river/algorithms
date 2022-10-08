struct NumMatrix {
	sums: Vec<Vec<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

	fn new(matrix: Vec<Vec<i32>>) -> Self {
		let n = matrix.len();
		let m = if n == 0 {
			0
		} else {
			matrix[0].len()
		};
		let mut sums = vec![vec![0; m + 1]; n + 1];
		for i in 1..=n {
			for j in 0..m {
				sums[i][j+1] = matrix[i-1][j] + sums[i][j];
			}
		}
		for j in 1..=m {
			for i in 0..n {
				sums[i+1][j] += sums[i][j];
			}
		}
		Self {
			sums
		}
	}

	fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
		let row1 = row1 as usize + 1;
		let col1 = col1 as usize + 1;
		let row2 = row2 as usize + 1;
		let col2 = col2 as usize + 1;
		self.sums[row2][col2] - self.sums[row1 - 1][col2] - self.sums[row2][col1 - 1] + self.sums[row1 - 1][col1 - 1]
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let matrix = [
			[3, 0, 1, 4, 2],
			[5, 6, 3, 2, 1],
			[1, 2, 0, 1, 5],
			[4, 1, 0, 1, 7],
			[1, 0, 3, 0, 5]
		];
		let matrix = matrix.to_vec().into_iter().map(|v| v.to_vec()).collect();
		let obj = NumMatrix::new(matrix);
		assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
		assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
		assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
	}
}

