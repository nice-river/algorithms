struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let people = vec![vec![7,0], vec![4,4], vec![7,1], vec![5,0], vec![6,1], vec![5,2]];
		Solution::reconstruct_queue(people);
	}
}


impl Solution {
	pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let mut people = people.clone();
		people.sort_by_key(|v| { (v[0], -v[1]) });
		let mut ans = Vec::with_capacity(people.len());
		for i in (0..people.len()).rev() {
			ans.insert(people[i][1] as usize, people[i].clone());
		}
		ans
	}
}