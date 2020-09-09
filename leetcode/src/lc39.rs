struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let candidates = vec![2, 3, 5, 7];
		let ans = Solution::combination_sum(candidates, 7);
		assert_eq!(ans.len(), 2);
	}
}

#[derive(Clone, PartialEq)]
struct Pair(usize, usize);

#[derive(Clone, PartialEq)]
enum State {
	Ans(Vec<Vec<Pair>>),
	NoAns,
	NotVis,
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
	pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
		let mut ans = vec![];
		let mut dp = vec![vec![State::NotVis; candidates.len()]; target as usize + 1];
		let target = target as usize;
		Solution::dfs(target, 0, &candidates, &mut dp);
		match &dp[target][0] {
			State::Ans(all) => {
				for one in all {
					ans.push(Vec::new());
					for p in one {
						let Pair(x, y) = p;
						for _ in 0..*y {
							ans.last_mut().unwrap().push(*x as i32);
						}
					}
				}
			}
			_ => {}
		}
		ans
	}

	fn dfs(target: usize, idx: usize, candidates: &Vec<i32>, dp: &mut Vec<Vec<State>>) {
		if dp[target][idx] != State::NotVis {
			return ;
		}
		if target == 0 {
			dp[target][idx] = State::Ans(vec![vec![]]);
			return ;
		}
		if idx + 1 == candidates.len() {
			if target % candidates[idx] as usize == 0 {
				dp[target][idx] = State::Ans(vec![vec![Pair(candidates[idx] as usize, target / candidates[idx] as usize)]]);
			} else {
				dp[target][idx] = State::NoAns;
			}
			return ;
		}
		dp[target][idx] = State::Ans(Vec::new());
		for i in 0..=(target / candidates[idx] as usize) {
			let nxt = target - i * candidates[idx] as usize;
			Solution::dfs(nxt, idx + 1, candidates, dp);

			let raw: *mut Vec<Vec<State>> = dp;

			unsafe {
				match &(*raw)[nxt][idx + 1] {
					State::Ans(v) => {
						if let State::Ans(d) = &mut (*raw)[target][idx] {
							if i == 0 {
								for p in v {
									d.push(p.clone());
								}
							} else {
								for p in v {
									d.push(p.clone());
									d.last_mut().unwrap().push(Pair(candidates[idx] as usize, i));
								}
							}
						}
					}
					_ => {},
				}
			}
		}
	}
}

