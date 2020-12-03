struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let intervals = [[1,5]].iter().map(|a| a.to_vec()).collect::<Vec<Vec<i32>>>();
		let new_interval = vec![2,7];
		assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1, 7]]);
	}
}

impl Solution {
	pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		if intervals.len() == 0 {
			ans.push(new_interval);
		} else {
			let mut i = 0;
			let mut f = false;
			if intervals[0][0] < new_interval[0] {
				ans.push(intervals[0].clone());
				i += 1;
			} else {
				ans.push(new_interval.clone());
				f = true;
			}
			while i < intervals.len() {
				let mut v;
				if !f {
					if intervals[i][0] < new_interval[0] {
						v = &intervals[i];
						i += 1;
					} else {
						f = true;
						v = &new_interval;
					}
				} else {
					v = &intervals[i];
					i += 1;
				}
				let l = ans.last().unwrap();
				if v[0] <= l[1] {
					(*ans.last_mut().unwrap())[1] = std::cmp::max(l[1], v[1]);
				} else {
					ans.push(v.clone());
				}
			}
			if !f {
				let v = &new_interval;
				let l = ans.last().unwrap();
				if v[0] <= l[1] {
					(*ans.last_mut().unwrap())[1] = std::cmp::max(l[1], v[1]);
				} else {
					ans.push(v.clone());
				}
			}
		}
		ans
	}
}