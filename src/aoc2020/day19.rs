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

#[derive(Clone)]
enum Type {
	Primitive(u8),
	Group(Vec<Vec<usize>>),
}

fn solution() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let mut rules = vec![Type::Primitive(0); 300];
	let mut ans = 0;
	let mut input_rules = true;
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let s = line.trim().to_string();
		if input_rules {
			if s.len() == 0 {
				input_rules = false;
			} else {
				let mut sp = s.split(":");
				let k = sp.next().unwrap().parse::<usize>().unwrap();
				let rule = sp.next().unwrap().trim().to_string();
				if rule.as_bytes()[0] == b'"' {
					rules[k] = Type::Primitive(rule.as_bytes()[1])
				} else {
					let mut sp = rule.split("|");
					let mut groups = Vec::new();
					while let Some(s) = sp.next() {
						let ch = s.trim().split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
						groups.push(ch);
					}
					rules[k] = Type::Group(groups);
				}
			}
		} else {
			let arr = s.as_bytes();
			let mut memo = HashMap::new();
			if helper(0, arr.len(), arr, 0, &rules, &mut memo) {
				ans += 1;
				dbg!(s);
			}
		}
		line.clear();
	}
	println!("{:?}", ans);
	Ok(())
}

fn helper(st: usize, ed: usize, arr: &[u8], rule_idx: usize, rules: &Vec<Type>, memo: &mut HashMap<(usize, usize, usize), bool>) -> bool {
	if let Some(e) = memo.get(&(st, ed, rule_idx)) {
		return *e;
	}
	let rule = &rules[rule_idx];
	let mut ret = false;
	match rule {
		Type::Primitive(c) => {
			ret = if ed - st != 1 {
				false
			} else {
				c == &arr[st]
			}
		}
		Type::Group(groups) => {
			'group: for group in groups {
				if foo(st, ed, arr, 0, group, rules, memo) {
					ret = true;
					break;
				}
			}
		}
	}
	memo.insert((st, ed, rule_idx), ret);
	ret
}

fn foo(st: usize, ed: usize, arr: &[u8], idx: usize, group: &Vec<usize>, rules: &Vec<Type>, memo: &mut HashMap<(usize, usize, usize), bool>) -> bool {
	if st == ed {
		return idx == group.len();
	}
	if idx == group.len() {
		return false;
	}
	for i in st+1..=ed {
		if helper(st, i, arr, group[idx], rules, memo) {
			if foo(i, ed, arr, idx + 1, group, rules, memo) {
				return true;
			}
		}
	}
	false
}