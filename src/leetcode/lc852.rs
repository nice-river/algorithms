struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut l = 1;
		let mut r = arr.len() - 2;
		while l + 1 < r {
            let m = l + (r - l) / 2;
			if arr[m-1] < arr[m] {
				l = m;
			} else {
				r = m;
			}
		}
        if l == r {
			l as i32
		} else {
			if arr[l] > arr[r] {
				l as i32
			} else {
				r as i32
			}
		}
    }
}