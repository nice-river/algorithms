struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut arr = nums1.clone();
		arr.sort_unstable();
		let mut diff = 0;
        let mut ans = 0;
		let m = 10i32.pow(9) + 7;
        for (&a, &b) in nums1.iter().zip(nums2.iter()) {
            let idx = lower_bound(&arr, 0, arr.len(), &b);
			let k = (a - b).abs();
			ans += k;
            ans %= m;
            if idx != arr.len() {
                let g = (arr[idx] - b).abs();
				if g < k && (k - g) > diff {
					ans += diff;
					ans += g - k;
					ans %= m;
					diff = k - g;
				}
			}
			if idx != 0 {
				let g = (arr[idx - 1] - b).abs();
				if g < k && (k - g) > diff {
					ans += diff;
					ans += g - k;
					ans %= m;
					diff = k - g;
				}
			}
		}
        ans
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
