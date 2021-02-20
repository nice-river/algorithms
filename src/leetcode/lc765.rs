struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let row = vec![0, 2, 1, 3];
		assert_eq!(Solution::min_swaps_couples(row), 1);

		let row = vec![3, 2, 0, 1];
		assert_eq!(Solution::min_swaps_couples(row), 0);

		let row = vec![4, 1, 0, 3, 2, 5];
		assert_eq!(Solution::min_swaps_couples(row), 2);
	}
}

impl Solution {
	pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
		let n = row.len();
		let mut p = vec![0; n];
		let mut q = vec![0; n];
		for (i, &x) in row.iter().enumerate() {
			p[x as usize] = i;
			q[i] = x as usize;
		}

		let mut ans = 0;
		for i in 0..row.len() {
			let mut times = 0;
			Solution::correct(i, &mut p, &mut q, &mut times);
			ans += times;
		}
		ans
	}

	fn correct(mut idx: usize, p: &mut Vec<usize>, q: &mut Vec<usize>, times: &mut i32) {
		if idx % 2 == 1 {
			idx -= 1;
		}
		if q[idx] > q[idx + 1] {
			p.swap(q[idx], q[idx + 1]);
			q.swap(idx, idx + 1);
		}
		if q[idx] % 2 == 0 {
			if q[idx + 1] == q[idx] + 1 {
				return ;
			}
			*times += 1;
			let g = q[idx + 1];
			let x = p[q[idx] + 1];
			q[idx + 1] = q[idx] + 1;
			q[x] = g;
			p[g] = x;
			p[q[idx] + 1] = idx + 1;
		} else {
			*times += 1;
			let g = q[idx + 1];
			let x = p[q[idx] - 1];
			q[idx + 1] = q[idx] - 1;
			q[x] = g;
			p[g] = x;
			p[q[idx] - 1] = idx + 1;

		}
	}

}