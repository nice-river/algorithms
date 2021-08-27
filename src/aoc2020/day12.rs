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
	let (mut x, mut y) = (0, 0);
	let mut dirs = [ [-1, 0], [0, 1], [1, 0], [0, -1] ];
	let mut cur_dir = 2i32;
	for act in arr {
		let act = act.as_bytes();
		let num = std::str::from_utf8(&act[1..]).unwrap().parse::<i32>().unwrap();
		match act[0] {
			b'F' => {
				x += num * dirs[cur_dir as usize][0];
				y += num * dirs[cur_dir as usize][1];
			}
			b'N' => {
				y += num;
			}
			b'S' => {
				y -= num;
			}
			b'W' => {
				x -= num;
			}
			b'E' => {
				x += num;
			}
			b'L' => {
				assert_eq!(num % 90, 0);
				let t = num / 90;
				cur_dir = (cur_dir - t + 4) % 4;
			}
			b'R' => {
				assert_eq!(num % 90, 0);
				let t = num / 90;
				cur_dir = (cur_dir + t + 4) % 4;
			}
			_ => unreachable!()
		}
	}
	x.abs() + y.abs()
}

fn helper2(arr: &Vec<String>) -> i32 {
	let (mut x, mut y) = (0, 0);
let mut dir = [10, 1];
	for act in arr {
		let act = act.as_bytes();
		let num = std::str::from_utf8(&act[1..]).unwrap().parse::<i32>().unwrap();
		match act[0] {
			b'F' => {
				x += num * dir[0];
				y += num * dir[1];
			}
			b'N' => {
				dir[1] += num;
			}
			b'S' => {
				dir[1] -= num;
			}
			b'W' => {
				dir[0] -= num;
			}
			b'E' => {
				dir[0] += num;
			}
			b'L' => {
				let t = ((num / 90) % 4 + 4) % 4;
				for _ in 0..t {
					let z = dir[0];
					dir[0] = -dir[1];
					dir[1] = z;
				}
			}
			b'R' => {
				let t = ((num / 90) % 4 + 4) % 4;
				for _ in 0..t {
					let z = dir[0];
					dir[0] = dir[1];
					dir[1] = -z;
				}
			}
			_ => unreachable!()
		}
	}
	x.abs() + y.abs()
}
