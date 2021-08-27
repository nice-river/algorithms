struct Solution {}

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
	let mut group = String::new();
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			if !group.is_empty() {
				ans += helper(&group);
			}
			break;
		}
		let s = line.trim_end();
		if s.is_empty() {
			ans += helper(&group);
			group.clear();
		} else {
			if group.is_empty() {
				group = s.to_string();
			} else {
				group = format!("{} {}", group, s);
			}
		}
		line.clear();
	}
	println!("{:?}", ans);
	Ok(())
}

fn helper(s: &str) -> i32 {
	let questions = s.split(" ").map(|e| e.as_bytes()).collect::<Vec<&[u8]>>();
	let mut ret = 0;
	for &b in questions[0].iter() {
		let mut f = true;
		for i in 1..questions.len() {
			if !questions[i].contains(&b) {
				f = false;
				break;
			}
		}
		if f {
			ret += 1;
		}
	}
	ret
}