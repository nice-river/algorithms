#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		solution();
	}

	#[test]
	fn test_ee() {
		let a = 12;
		let b = 5;
		let (mut x, mut y) = (0, 0);
		let g = extended_euclidean(a, b, &mut x, &mut y);
		let m = a * b / g;
		while x > 0 {
			x -= b / g;
			y += a / g;
		}
		dbg!(x, y);
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
	println!("{:?}", helper2(&arr));
	Ok(())
}

fn helper(arr: &Vec<String>) -> i32 {
	let timestamp = arr[0].parse::<i32>().unwrap();
	let mut mini = std::i32::MAX;
	let mut ans = 0;
	for s in arr[1].split(",") {
		if s == "x" {
			continue;
		}
		let bus_id = s.parse::<i32>().unwrap();
		let t = timestamp / bus_id + if timestamp % bus_id != 0 {1} else {0};
		if t * bus_id - timestamp < mini {
			mini = t * bus_id - timestamp;
			ans = mini * bus_id;
		}
	}
	ans
}

fn helper2(arr: &Vec<String>) -> i64 {
	let arr: Vec<i64> = arr[1].split(",").into_iter().map(|s| if s == "x" { 0 } else {s.parse::<i64>().unwrap()}).collect();
	let (mut x, mut y) = (0, 0);
	let mut a = arr[0] as i64;
	let mut k = 0;
	for i in 1..arr.len() {
		if arr[i] == 0 { continue; }
		let mut g = extended_euclidean(a, arr[i] as i64, &mut x, &mut y);
		let t = arr[i] / g;
		if x > 0 {
			x -= t * (x / t + 1);
		} else {
			x -= t * (x / t);
		}
		x = (x * (-(i as i64) - k) / g) % t;
		k = k + x * a;
		a = a * arr[i] / g;
	}
	k
}


fn extended_euclidean(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
	if b == 0 {
		*x = 1;
		*y = 0;
		a
	} else {
		let g = extended_euclidean(b, a % b, x, y);
		let tmp = *x;
		*x = *y;
		*y = tmp - a / b * *y;
		g
	}
}