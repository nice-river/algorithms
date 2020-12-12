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

fn solution() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let mut ans = 0;

	let mut nums = vec![];
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let s = line.trim().parse::<usize>().unwrap();
		nums.push(s);

		line.clear();
	}

	println!("{:?}", helper2(&nums, 1038347917));
	Ok(())
}

// 1038347917
fn helper(arr: &Vec<usize>, target: usize) -> bool {
	for i in 0..arr.len() {
		for j in i+1..arr.len() {
			if arr[i] + arr[j] == target {
				return true;
			}
		}
	}
	false
}

fn helper2(arr: &Vec<usize>, target: usize) -> usize {
	let mut t = 0;
	for i in 0..arr.len() {
		t = arr[i];
		let (mut mini, mut maxi) = (t, t);
		for j in i+1..arr.len() {
			mini = std::cmp::min(mini, arr[j]);
			maxi = std::cmp::max(maxi, arr[j]);
			t += arr[j];
			if t == target {
				return mini + maxi;
			}
		}
	}
	return 0;
}