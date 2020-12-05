struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


use std::collections::HashMap;

impl Solution {
	pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
		let mut map = HashMap::new();
		for &task in &tasks {
			*map.entry(task).or_insert(0) += 1;
		}
		let (c, mut maxi) = map.iter().max_by_key(|(&k, &v)| v).unwrap();
		let mut base = *maxi * (n + 1) - n;
		let mut n = n;
		for (k, &v) in &map {
			if n == 0 { break; }
			if k != c && v == *maxi {
				base += 1;
				n -= 1;
			}
		}
		std::cmp::max(base, tasks.len() as i32)
	}
}