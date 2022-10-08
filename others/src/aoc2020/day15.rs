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
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let s = line.trim().to_string();
		println!("{:?}", helper(s));
		line.clear();
	}
	Ok(())
}

fn helper(s: String) -> usize {
	let arr: Vec<usize> = s.split(",").into_iter().map(|e| e.parse::<usize>().unwrap()).collect();
	let mut map = HashMap::new();
	for (i, &num) in arr.iter().enumerate() {
		map.insert(num, i + 1);
	}
	let mut nxt = 0;
	for i in arr.len()+1..30000000 {
		let mut tmp = nxt;
		match map.get(&nxt) {
			Some(&v) => {
				tmp = i - v;
			}
			None => {
				tmp = 0;
			}
		}
		map.insert(nxt, i);
		nxt = tmp;
	}
	nxt
}
