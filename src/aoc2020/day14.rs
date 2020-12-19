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

fn helper(arr: &Vec<String>) -> i64 {
	let mut map = HashMap::new();
	let mut mask= vec![];
	for s in arr.iter() {
		let mut sp: Vec<&str> = s.split(" = ").collect();
		if sp[0] == "mask" {
			mask = sp[1].to_string().into_bytes();
			mask.reverse();
		} else {
			let mut v: i64 = sp[1].parse().unwrap();
			for i in 0..mask.len() {
				match mask[i] {
					b'1' => v |= 1 << i,
					b'0' => v &= !(1 << i),
					_ => {}
				}
			}
			*map.entry(sp[0].to_string()).or_insert(0) = v;
		}
	}
	map.values().sum()
}

fn helper2(arr: &Vec<String>) -> i64 {
	let mut map = HashMap::new();
	let mut mask= vec![];
	for s in arr.iter() {
		let mut sp: Vec<&str> = s.split(" = ").collect();
		if sp[0] == "mask" {
			mask = sp[1].to_string().into_bytes();
			mask.reverse();
		} else {
			let v: i64 = sp[1].parse().unwrap();
			let c = sp[0].as_bytes();
			let mem = std::str::from_utf8(&c[4..c.len()-1]).unwrap().parse::<i64>().unwrap();
			f(mem, v, &mask, 0, &mut map);
		}
	}
	map.values().sum()
}

fn f(mem: i64, v: i64, mask: &Vec<u8>, idx: usize, map: &mut HashMap<i64, i64>) {
	if idx == mask.len() {
		map.insert(mem, v);
		return ;
	}
	match mask[idx] {
		b'X' => {
			f(mem | (1 << idx), v, mask, idx + 1, map);
			f(mem & !(1 << idx), v, mask, idx + 1, map);
		}
		b'1' => {
			f(mem | (1 << idx), v, mask, idx + 1, map);
		}
		b'0' => {
			f(mem, v, mask, idx + 1, map);
		}
		_ => {}
	}
}
