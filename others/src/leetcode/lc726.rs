struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let formula = "H2O".to_string();
		let ans = "H2O".to_string();
		assert_eq!(Solution::count_of_atoms(formula), ans);
	}

	#[test]
	fn test1() {
		let formula = "Mg(OH)2".to_string();
		let ans = "H2MgO2".to_string();
		assert_eq!(Solution::count_of_atoms(formula), ans);
	}

	#[test]
	fn test2() {
		let formula = "K4(ON(SO3)2)2".to_string();
		let ans = "K4N2O14S4".to_string();
		assert_eq!(Solution::count_of_atoms(formula), ans);
	}

	#[test]
	fn test3() {
		let formula = "Be32".to_string();
		let ans = "Be32".to_string();
		assert_eq!(Solution::count_of_atoms(formula), ans);
	}
}

use std::collections::HashMap;

impl Solution {
	pub fn count_of_atoms(formula: String) -> String {
        let formula = formula.into_bytes();
        let (_, map) = Solution::dfs(&formula, 0);
        let mut arr = map.into_iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
        arr.sort_unstable();
        arr.into_iter().map(|(a, b)| {
			if b > 1 {
				format!("{}{}", a, b)
			} else {
				a
			}
		}).collect::<Vec<_>>().join("")
	}

	fn dfs(formula: &Vec<u8>, mut idx: usize) -> (usize, HashMap<String, i32>) {
		let mut map = HashMap::new();
        let mut cur_atomic = String::new();
        while idx < formula.len() {
			match formula[idx] {
				b'(' => {
                    let (nxt_idx, ret) = Solution::dfs(formula, idx + 1);
					idx = nxt_idx;
					Solution::join_map(&mut map, &ret);
				}
                b'A'..=b'Z' => {
					if cur_atomic.len() != 0 {
						*map.entry(cur_atomic).or_insert(0) += 1;
					}
                    let (nxt_idx, atomic) = Solution::fetch_atomic(formula, idx);
					cur_atomic = atomic;
                    idx = nxt_idx;
				}
				b'0'..=b'9' => {
					let (nxt_idx, cnt) = Solution::fetch_num(formula, idx);
					*map.entry(cur_atomic).or_insert(0) += cnt;
					cur_atomic = String::new();
					idx = nxt_idx;
				}
				b')' => {
					if cur_atomic.len() != 0 {
						*map.entry(cur_atomic).or_insert(0) += 1;
					}
					let (nxt_idx, cnt) = Solution::fetch_num(formula, idx + 1);
					for (_, v) in map.iter_mut() {
                        *v *= cnt;
					}
					return (nxt_idx, map);
				}
				_ => {
					idx += 1;
				}
			}
		}
		if cur_atomic.len() != 0 {
			*map.entry(cur_atomic).or_insert(0) += 1;
		}
		(idx, map)
	}

	fn join_map(dst: &mut HashMap<String, i32>, src: &HashMap<String, i32>) {
		for (k, &v) in src.iter() {
            if let Some(x) = dst.get_mut(k) {
				*x += v;
			} else {
				dst.insert(k.clone(), v);
			}
		}
	}

	fn fetch_num(formula: &Vec<u8>, mut idx: usize) -> (usize, i32) {
		let mut ret = 0;
        while idx < formula.len() && b'0' <= formula[idx] && formula[idx] <= b'9' {
			ret = ret * 10 + (formula[idx] - b'0') as i32;
			idx += 1;
		}
		(idx, ret.max(1))
	}

	fn fetch_atomic(formula: &Vec<u8>, idx: usize) -> (usize, String) {
		let mut j = idx + 1;
        while j < formula.len() && b'a' <= formula[j] && formula[j] <= b'z' {
			j += 1;
		}
		(j, String::from_utf8_lossy(&formula[idx..j]).to_string())
	}
}