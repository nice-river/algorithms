struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::BinaryHeap;
use std::cmp::Ordering;

impl Solution {
	pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
		let mut heap = BinaryHeap::new();
		for point in points {
			heap.push(Wrapper(point));
			if heap.len() > k as usize {
				heap.pop();
			}
		}
		return heap.into_iter().map(|e| e.0).collect()
	}
}

struct Wrapper(Vec<i32>);

impl std::cmp::PartialOrd for Wrapper {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let a = self.0[0] * self.0[0] + self.0[1] * self.0[1];
		let b = other.0[0] * other.0[0] + other.0[1] * other.0[1];
		Some(a.cmp(&b))
	}
}

impl std::cmp::Ord for Wrapper {
	fn cmp(&self, other: &Self) -> Ordering {
		let a = self.0[0] * self.0[0] + self.0[1] * self.0[1];
		let b = other.0[0] * other.0[0] + other.0[1] * other.0[1];
		a.cmp(&b)
	}
}

impl std::cmp::PartialEq for Wrapper {
	fn eq(&self, other: &Self) -> bool {
		let a = self.0[0] * self.0[0] + self.0[1] * self.0[1];
		let b = other.0[0] * other.0[0] + other.0[1] * other.0[1];
		a == b
	}
}

impl std::cmp::Eq for Wrapper {}