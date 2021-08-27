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

fn solution() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let mut ans = 0;

	let mut instructions = vec![];
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let s = line.trim().to_string();
		instructions.push(s);
		line.clear();
	}

	let mut vis = HashSet::new();
	helper(0, &mut ans, &instructions, false, &mut vis);
	println!("{:?}", ans);
	Ok(())
}

fn helper(pc: usize, ans: &mut i32, s: &Vec<String>, changed: bool, vis: &mut HashSet<usize>) -> bool {
	if pc >= s.len() {
		return true;
	}

	if vis.contains(&pc) {
		return false;
	}

	vis.insert(pc);

	let mut v = s[pc].split(" ").collect::<Vec<&str>>();
	let num = v[1].parse::<i32>().unwrap();
	match v[0] {
		"nop" => {
			if changed {
				if helper(pc + 1, ans, s, changed, vis) {
					return true;
				}
			} else {
				if helper(pc + 1, ans, s, changed, vis) {
					return true
				} else {
					let pc = std::cmp::max(pc as i32 + num, 0);
					if helper(pc as usize, ans, s, true, vis) {
						return true;
					}
				}
			}
		}
		"jmp" => {
			if changed {
				let pc = std::cmp::max(pc as i32 + num, 0);
				if helper(pc as usize, ans, s, changed, vis) {
					return true;
				}
			} else {
				if helper(pc + 1, ans, s, true, vis) {
					return true;
				} else {
					let pc = std::cmp::max(pc as i32 + num, 0);
					if helper(pc as usize, ans, s, changed, vis) {
						return true;
					}
				}
			}
		}
		"acc" => {
			*ans += num;
			if !helper(pc + 1, ans, s, changed, vis) {
				*ans -= num;
			} else {
				return true;
			}
		}
		_ => unreachable!()
	}
	vis.remove(&pc);
	false
}