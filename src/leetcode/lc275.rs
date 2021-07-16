struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut l = 0;
		let mut r = citations.len() + 1;
		while l + 1 < r {
            let m = l + (r - l) / 2;
			if citations[citations.len() - m] >= m as i32 {
				l = m;
			} else {
				r = m;
			}
		}
		l as i32
    }
}