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
use std::collections::HashMap;

fn solution() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let mut trees = vec![];
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 { break; }
		let c = line.trim().to_string();
		trees.push(c.into_bytes());
		line.clear();
	}
	let steps = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
	let mut ans = 1;
	for step in steps.iter() {
		ans *= helper(&trees, step[0], step[1]);
	}
	println!("{}", ans);
	Ok(())
}

fn helper(trees: &Vec<Vec<u8>>, right: usize, down: usize) -> usize {
	let mut p = 0;
	let mut ans = 0;
	for tree in trees.iter().step_by(down) {
		if tree[p] == b'#' {
			ans += 1;
		}
		p += right;
		p %= tree.len();
	}
	ans
}