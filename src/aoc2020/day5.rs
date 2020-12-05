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
	let mut arr = Vec::new();
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let s = line.trim_end();
		arr.push(helper(s));
		line.clear();
	}
	arr.sort();
	println!("{:?}", arr);
	Ok(())
}

fn helper(s: &str) -> i32 {
	let s = s.as_bytes();
	let (mut l, mut r) = (0, 128);
	for i in 0..7 {
		let m = (r + l) / 2;
		if s[i] == b'F' {
			r = m;
		} else {
			l = m;
		}
	}
	let ans = l * 8;
	l = 0;
	r = 8;
	for i in 7..10 {
		let m = (r + l) / 2;
		if s[i] == b'R' {
			l = m;
		} else {
			r = m;
		}
	}
	ans + l
}