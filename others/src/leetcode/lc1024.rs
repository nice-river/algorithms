struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let clips = vec![vec![0,0],vec![9,9],vec![2,10],vec![0,3],vec![0,5],vec![3,4],vec![6,10],vec![1,2],vec![4,7],vec![5,6]];
		let t = 5;
		Solution::video_stitching(clips, t);
	}
}

impl Solution {
	pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
		let t = t as usize;
		let mut memo = vec![0; t];
		for clip in clips.iter() {
			if clip[0] < memo.len() as i32 {
				memo[clip[0] as usize] = std::cmp::max(memo[clip[0] as usize], clip[1]);
			}
		}
		let mut ans = 0;
		let mut last = 0;
		let mut pre = 0;
		for i in 0..memo.len() {
			last = std::cmp::max(last,memo[i]);
			if i as i32 == last {
				return -1;
			}
			if i == pre {
				pre = last as usize;
				ans += 1;
			}
		}
		ans
	}
}