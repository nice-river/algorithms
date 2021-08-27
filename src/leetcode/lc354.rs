struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let envelopes = [[5,4],[6,4],[6,7],[2,3]];
		let envelopes = envelopes.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::max_envelopes(envelopes), 3);
	}

	#[test]
	fn test2() {
		let envelopes = [[4,5],[4,6],[6,7],[2,3],[1,1]];
		let envelopes = envelopes.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::max_envelopes(envelopes), 4);
	}

	#[test]
	fn test3() {
		let envelopes = [[1, 1], [1, 2]];
		let envelopes = envelopes.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::max_envelopes(envelopes), 1);
	}

	#[test]
	fn test4() {
		let envelopes = [[2,100],[3,200],[4,300],[5,500],[5,400],[5,250],[6,370],[6,360],[7,380]];
		let envelopes = envelopes.to_vec().into_iter().map(|v| v.to_vec()).collect();
		assert_eq!(Solution::max_envelopes(envelopes), 5);
	}
}

impl Solution {
	pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
		if envelopes.is_empty() {
			return 0;
		}
		envelopes.sort_unstable_by(|a, b| {
			if a[0] == b[0] {
				(-a[1]).cmp(&-b[1])
			} else {
				a[0].cmp(&b[0])
			}
		});

		let mut arr = vec![envelopes[0][1]];
		for i in 1..envelopes.len() {
			let t = envelopes[i][1];
			if &t > arr.last().unwrap() {
				arr.push(t);
			} else {
				let p = lower_bound(&arr, 0, arr.len(), &t);
				arr[p] = t;
			}
		}
		arr.len() as i32
	}
}


fn lower_bound<T>(arr: &[T], start: usize, end: usize, target: &T) -> usize
where
	T: Ord
{
	let (mut start, mut end) = (start, end);
	while start < end {
		let mid = start + (end - start) / 2;
		match arr[mid].cmp(target) {
			std::cmp::Ordering::Less => {
				start = mid + 1;
			}
			_ => {
				end = mid;
			}
		}
	}
	start
}
