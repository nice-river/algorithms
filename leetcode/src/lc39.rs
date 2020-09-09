struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let candidates = vec![2];
		let ans = Solution::combination_sum(candidates, 2);
		assert_eq!(ans.len(), 1);
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
		let dp = Rc::new(RefCell::new(vec![vec![State::NotVis; candidates.len()]; target as usize + 1]));
		let target = target as usize;
		Solution::dfs(target, 0, &candidates, dp.clone());
		match &(*RefCell::borrow(&dp))[target][0] {
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

	fn dfs(target: usize, idx: usize, candidates: &Vec<i32>, dp: Rc<RefCell<Vec<Vec<State>>>>) {
		if (*RefCell::borrow(&dp))[target][idx] != State::NotVis {
			return ;
		}
		if target == 0 {
			(*RefCell::borrow_mut(&dp))[target][idx] = State::Ans(vec![vec![]]);
			return ;
		}
		if idx + 1 == candidates.len() {
			if target % candidates[idx] as usize == 0 {
				(*RefCell::borrow_mut(&dp))[target][idx] = State::Ans(vec![vec![Pair(candidates[idx] as usize, target / candidates[idx] as usize)]]);
			} else {
				(*RefCell::borrow_mut(&dp))[target][idx] = State::NoAns;
			}
			return ;
		}
		(*RefCell::borrow_mut(&dp))[target][idx] = State::Ans(Vec::new());
		for i in 0..=(target / candidates[idx] as usize) {
			let nxt = target - i * candidates[idx] as usize;
			Solution::dfs(nxt, idx + 1, candidates, dp.clone());

			let raw: *mut _ = &mut (RefCell::borrow_mut(&dp));

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

