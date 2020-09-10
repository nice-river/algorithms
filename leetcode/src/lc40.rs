struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let ans = Solution::combination_sum2(vec![1, 1], 2);
		assert_eq!(ans.len(), 1)
	}
}

#[derive(Clone, PartialEq)]
enum State {
	Ans(Vec<Vec<i32>>),
	NoAns,
	NotVis,
}

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::alloc::handle_alloc_error;

impl Solution {
	pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
		let mut dp = vec![vec![State::NotVis; candidates.len()]; target as usize + 1];
		let mut counter = HashMap::new();
		for c in candidates.iter() {
			*counter.entry(*c).or_insert(0) += 1
		}
		let candidates = counter.keys().map(|v| *v).collect();
		let target = target as usize;
		Solution::dfs(target, 0, &candidates, &mut dp, &counter);
		match &dp[target][0] {
			State::Ans(all) => {
				all.clone()
			}
			_ => {
				vec![]
			}
		}
	}

	fn dfs(target: usize, idx: usize, candidates: &Vec<i32>, dp: &mut Vec<Vec<State>>, counter: &HashMap<i32, i32>) {
		if dp[target][idx] != State::NotVis {
			return ;
		}
		if target == 0 {
			dp[target][idx] = State::Ans(vec![vec![]]);
			return ;
		}
		if idx + 1 == candidates.len() {
			if target % candidates[idx] as usize == 0 && target / candidates[idx] as usize <= *counter.get(&candidates[idx]).unwrap() as usize {
				dp[target][idx] = State::Ans(vec![vec![candidates[idx]; target / candidates[idx] as usize]]);
			} else {
				dp[target][idx] = State::NoAns;
			}
			return ;
		}
		dp[target][idx] = State::Ans(Vec::new());

		for i in 0..=min(*counter.get(&candidates[idx]).unwrap() as usize, (target / candidates[idx] as usize)) {
			let nxt = target - i * candidates[idx] as usize;
			Solution::dfs(nxt, idx + 1, candidates, dp, counter);
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
									for _ in 0..i {
										d.last_mut().unwrap().push(candidates[idx]);
									}
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
