use std::fs::{File, read};
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		solution2();
	}
}


fn solution() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let mut ans = 0;
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let mut splits = line.split(" ");
		let rng: Vec<i32> = splits.next().unwrap().split("-").map(|e| e.parse::<i32>().unwrap()).collect();
		let letter = splits.next().unwrap().as_bytes()[0];
		let mut counter = 0;
		for &e in splits.next().unwrap().as_bytes() {
			if e == letter {
				counter += 1;
			}
		}
		if rng[0] <= counter && counter <= rng[1] {
			ans += 1;
		}
		line.clear();
	}
	println!("{}", ans);
	Ok(())
}

fn solution2() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let mut ans = 0;
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let mut splits = line.split(" ");
		let pos: Vec<i32> = splits.next().unwrap().split("-").map(|e| e.parse::<i32>().unwrap()).collect();
		let letter = splits.next().unwrap().as_bytes()[0];
		let mut counter = 0;
		let arr = splits.next().unwrap().as_bytes();
		for p in pos {
			if arr[p as usize - 1] == letter {
				counter += 1;
			}
		}
		if counter == 1 {
			ans += 1;
		}
		line.clear();
	}
	println!("{}", ans);
	Ok(())
}
