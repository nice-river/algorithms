struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![5,7,7,8,8,10];
		assert_eq!(upper_bound(&nums, 0, nums.len(), &6), 1);
	}
}


impl Solution {
	pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
		let p = lower_bound(&nums, 0, nums.len(), &target);
		if p != nums.len() && nums[p] == target {
			let n= upper_bound(&nums, 0, nums.len(), &target);
			vec![p as i32, n as i32 - 1]
		} else {
			vec![-1, -1]
		}
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

fn upper_bound<T>(arr: &[T], start: usize, end: usize, target: &T) -> usize
where
	T: Ord
{
	let (mut start, mut end) = (start, end);
	while start < end {
		let mid = start + (end - start) / 2;
		match arr[mid].cmp(target) {
			std::cmp::Ordering::Greater => {
				end = mid;
			}
			_ => {
				start = mid + 1;
			}
		}
	}
	start
}
