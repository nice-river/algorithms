struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let matrix = [[1,2,3],[4,5,6],[7,8,9]];
		let matrix = matrix.to_vec().into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::spiral_order(matrix), vec![1,2,3,6,9,8,7,4,5]);
	}

	#[test]
	fn test2() {
		let matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]];
		let matrix = matrix.to_vec().into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::spiral_order(matrix), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
	}

	#[test]
	fn test3() {
		let matrix = [[1,2,3,4]];
		let matrix = matrix.to_vec().into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::spiral_order(matrix), vec![1, 2, 3, 4]);
	}

	#[test]
	fn test4() {
		let matrix = [[1], [2], [3], [4]];
		let matrix = matrix.to_vec().into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::spiral_order(matrix), vec![1, 2, 3, 4]);
	}

	#[test]
	fn test5() {
		let matrix = vec![];
		// let matrix = matrix.to_vec().into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::spiral_order(matrix), vec![]);

		let matrix = [[]];
		let matrix = matrix.to_vec().into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::spiral_order(matrix), vec![]);

		let matrix = [[1]];
		let matrix = matrix.to_vec().into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::spiral_order(matrix), vec![1]);
	}
}

enum Dir {
	Left,
	Right,
	Up,
	Down,
}


impl Solution {
	pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
		if matrix.is_empty() || matrix[0].is_empty() {
			return vec![];
		}
		let mut ans = vec![];
		let mut t = 0;
		let mut b = matrix.len() as i32 - 1;
		let mut l = 0;
		let mut r = matrix[0].len() as i32 - 1;
		let (mut x, mut y) = (0, 0);
		let mut cur_dir = Dir::Right;
		while t <= x && x <= b && l <= y && y <= r {
			ans.push(matrix[x as usize][y as usize]);
			Solution::forward(&mut x, &mut y, &mut l, &mut r, &mut t, &mut b, &mut cur_dir);
		}
		ans
	}

	fn forward(x: &mut i32, y: &mut i32, l: &mut i32, r: &mut i32, t: &mut i32, b: &mut i32, cur_dir: &mut Dir) {
		use Dir::*;
		match cur_dir {
			Right => {
				if y == r {
					*t += 1;
					*x += 1;
					*cur_dir = Down;
				} else {
					*y += 1;
				}
			}
			Down => {
				if x == b {
					*r -= 1;
					*y -= 1;
					*cur_dir = Left;
				} else {
					*x += 1;
				}
			}
			Left => {
				if y == l {
					*b -= 1;
					*x -= 1;
					*cur_dir = Up;
				} else {
					*y -= 1;
				}
			}
			Up => {
				if x == t {
					*l += 1;
					*y += 1;
					*cur_dir = Right;
				} else {
					*x -= 1;
				}
			}
		}
	}
}