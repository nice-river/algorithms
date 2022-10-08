#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut cnt = vec![0; 11];
		for _ in 0..10000 {
			cnt[Solution::rand10() as usize] += 1;
		}
		println!("{:?}", cnt);
	}
}

struct Solution {}

fn rand7() -> i32 {
	(rand::random::<u32>() % 7) as i32 + 1
}

impl Solution {
	pub fn rand10() -> i32 {
		loop {
			let x = (rand7() - 1) * 7 + rand7() - 1;
			if x < 40 {
				return x % 10 + 1;
			}
		}
	}
}