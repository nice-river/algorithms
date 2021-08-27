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
		arr.push(s.into_bytes());
		line.clear();
	}
	println!("{:?}", helper(&arr));
	Ok(())
}

fn get_state(x: i32, y: i32, z: i32, w: i32, layers: &Vec<Vec<Vec<Vec<u8>>>>) -> u8 {
	if x <= -1 || y <= -1 || z <= -1 || w <= -1 {
		return b'.';
	}
	let l = layers.len() as i32;
	let n = layers[0].len() as i32;
	let m = layers[0][0].len() as i32;
	let o = layers[0][0][0].len() as i32;
	if x >= l || y >= n || z >= m || w >= o {
		b'.'
	} else {
		layers[x as usize][y as usize][z as usize][w as usize]
	}
}

fn helper(arr: &Vec<Vec<u8>>) -> i64 {
	let mut layers = vec![vec![arr.clone()]];
	let cycles = 6;
	for _ in 0..cycles {
		let mut nxt = vec![];
		let n = layers[0].len() as i32;
		let m = layers[0][0].len() as i32;
		let o = layers[0][0][0].len() as i32;
		for layer in -1..=layers.len() as i32 {
			let mut new = vec![vec![vec![0; o as usize + 2]; m as usize + 2]; n as usize + 2];
			for i in -1..=n {
				for j in -1..=m {
					for k in -1..=o {
						let cur = get_state(layer, i, j, k, &layers);
						let mut active_neighbors = 0;
						'outer: for x in -1..=1 {
							for y in -1..=1 {
								for z in -1..=1 {
									for w in -1..=1 {
										if x == 0 && y == 0 && z == 0 && w == 0{
											continue;
										}
										let neighbor = get_state(layer + x, i + y, j + z, k + w, &layers);
										if neighbor == b'#' {
											active_neighbors += 1;
										}
										if active_neighbors >= 4 {
											break 'outer;
										}
									}
								}
							}
						}

						new[(i + 1) as usize][(j + 1) as usize][(k + 1) as usize] =
								if cur == b'.' {
									if active_neighbors == 3 {
										b'#'
									} else {
										b'.'
									}
								} else {
									if active_neighbors == 2 || active_neighbors == 3 {
										b'#'
									} else {
										b'.'
									}
								}
					}
				}
			}
			nxt.push(new);
		}
		layers = nxt;
	}
	let mut ans = 0;
	for x in layers {
		for y in x {
			for z in y {
				for w in z {
					if w == b'#' {
						ans += 1;
					}
				}
			}
		}
	}
	ans
}
