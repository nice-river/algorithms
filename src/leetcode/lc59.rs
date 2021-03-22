struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}

}

enum Dir {
	Left,
	Right,
	Up,
	Down,
}


impl Solution {
	pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
		if n == 0 {
			return vec![];
		}
		let mut ans = vec![vec![0; n as usize]; n as usize];
		let mut t = 0;
		let mut b = n - 1;
		let mut l = 0;
		let mut r = n - 1;
		let (mut x, mut y) = (0, 0);
		let mut cur_dir = Dir::Right;
		let mut i = 1;
		while t <= x && x <= b && l <= y && y <= r {
			ans[x as usize][y as usize] = i;
			i += 1;
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