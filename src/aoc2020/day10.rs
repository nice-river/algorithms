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
	println!("{:?}", helper2(&nums));
	Ok(())
}

fn helper(nums: &Vec<usize>) -> usize {
	let mut map = HashMap::new();
	for &num in nums.iter() {
		*map.entry(num).or_insert(0) += 1;
	}
	let mut a = 0;
	let mut b = 1;
	let mut cur = 0;
	loop {
		if map.contains_key(&(cur + 1)) {
			cur += 1;
			a += 1;
		} else if map.contains_key(&(cur + 2)) {
			cur += 2;
		} else if map.contains_key(&(cur + 3)) {
			cur += 3;
			b += 1;
		} else {
			break;
		}
	}
	a * b
}

fn helper2(nums: &Vec<usize>) -> usize {
	let mut map = HashMap::new();
	let mut maxi = 0;
	for &num in nums.iter() {
		maxi = std::cmp::max(maxi, num);
		*map.entry(num).or_insert(0) += 1;
	}
	let mut memo = vec![0; maxi + 1];
	memo[0] = 1;
	for i in 0..=maxi {
		if memo[0] != 0{
			for j in 1..=3 {
				if i + j <= maxi && map.contains_key(&(i + j)) {
					memo[i + j] += memo[i];
				}
			}
		}
	}
	memo[maxi]
}
