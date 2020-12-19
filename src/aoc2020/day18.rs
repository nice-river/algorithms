#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		solution();
	}
}

use std::fs::{File, read};
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::cmp::max;
use std::iter::FromIterator;

fn solution() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let mut arr = vec![];
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let s = line.trim().to_string();
		arr.push(s);
		line.clear();
	}
	println!("{:?}", helper(arr));
	Ok(())
}

fn helper(arr: Vec<String>) -> i64 {
	let mut ans = 0;
	let mut map = HashMap::new();
	let mut stk = vec![];
	for line in arr {
		let line = line.into_bytes();
		for i in 0..line.len() {
			match line[i] {
				b'(' => {
					stk.push(i);
				},
				b')' => {
					map.insert(stk.pop().unwrap(), i);
				},
				_ => {}
			};
		}
		let r = eval(0, line.len(), &line, &map);
		dbg!(r);
		ans += r;
		stk.clear();
		map.clear();
	}
	ans
}

fn eval(st: usize, ed: usize, expr: &Vec<u8>, parentheses_match: &HashMap<usize, usize>) -> i64 {
	let mut idx = st;
	let mut nums = vec![];
	let mut ops = vec![];
	while idx < ed {
		let (num, nxt_idx) = get_next_num(idx, expr, parentheses_match);
		idx = nxt_idx;
		nums.push(num);
		let (op, nxt_idx) = get_next_op(idx, ed, expr);
		if op == 0 {
			break;
		}
		idx = nxt_idx;
		ops.push(op);
	}
	// let mut ret = nums[0];
	// for i in 0..ops.len() {
	// 	match ops[i] {
	// 		b'+' => {
	// 			ret += nums[i + 1];
	// 		},
	// 		b'*' => {
	// 			ret *= nums[i + 1];
	// 		}
	// 		_ => unreachable!()
	// 	}
	// }
	// ret
	let mut muls = vec![nums[0]];
	for i in 0..ops.len() {
		match ops[i] {
			b'+' => {
				let t = muls.pop().unwrap() + nums[i + 1];
				muls.push(t);
			},
			b'*' => {
				muls.push(nums[i + 1]);
			}
			_ => unreachable!()
		}
	}
	muls.into_iter().fold(1, |acc, n| acc * n)
}

fn get_next_num(idx: usize, expr: &Vec<u8>, parentheses_match: &HashMap<usize, usize>) -> (i64, usize) {
	let mut i = idx;
	while i < expr.len() {
		if expr[i] == b'(' {
			return (eval(i + 1, parentheses_match[&i], expr, parentheses_match), parentheses_match[&i] + 1);
		} else if b'0' <= expr[i] && expr[i] <= b'9' {
			let mut j = i + 1;
			while j < expr.len() && b'0' <= expr[j] && expr[j] <= b'9' {
				j += 1;
			}
			return (std::str::from_utf8(&expr[i..j]).unwrap().parse::<i64>().unwrap(), j);
		}
		i += 1;
	}
	(0, expr.len())
}

fn get_next_op(idx: usize, ed: usize, expr: &Vec<u8>) -> (u8, usize) {
	let mut i = idx;
	while i < ed {
		if expr[i] == b'*' || expr[i] == b'+' {
			return (expr[i], i + 1);
		}
		i += 1;
	}
	(0, ed)
}
