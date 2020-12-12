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

	let mut board = vec![];

	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let s: Vec<char> = line.trim().chars().collect();
		board.push(s);
		line.clear();
	}
	println!("{:?}", helper2(&mut board));
	Ok(())
}

fn helper(board: &mut Vec<Vec<char>>) -> usize {
	let mut bs = [board.clone(), board.clone()];
	let mut ans = 0;
	let mut cur = 0;
	let (n, m) = (board.len(), board[0].len());
	loop {
		let mut changed = false;
		let nxt = cur ^ 1;
		for i in 0..n {
			for j in 0..m {
				match bs[cur][i][j] {
					'.' => {
						bs[nxt][i][j] = '.';
					}
					'L' => {
						let mut f = false;
						for x in -1..=1 {
							for y in -1..=1 {
								let nx = i as i32 + x;
								let ny = j as i32 + y;
								if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
									continue;
								}
								if nx as usize == i && ny as usize == j {
									continue;
								}
								if bs[cur][nx as usize][ny as usize] == '#' {
									f = true;
									break;
								}
							}
						}
						if !f {
							bs[nxt][i][j] = '#';
							changed = true;
						} else {
							bs[nxt][i][j] = 'L';
						}
					}
					'#' => {
						let mut c = 0;
						for x in -1..=1 {
							for y in -1..=1 {
								let nx = i as i32 + x;
								let ny = j as i32 + y;
								if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
									continue;
								}
								if nx as usize == i && ny as usize == j {
									continue;
								}
								if bs[cur][nx as usize][ny as usize] == '#' {
									c += 1;
								}
							}
						}
						if c >= 4 {
							bs[nxt][i][j] = 'L';
							changed = true;
						} else {
							bs[nxt][i][j] = '#';
						}
					}
					_ => unreachable!()
				}
			}
		}
		if !changed {
			break;
		}
		cur ^= 1;
	}
	for i in 0..n {
		for j in 0..m {
			if bs[cur][i][j] == '#' {
				ans += 1;
			}
		}
	}
	ans
}


fn helper2(board: &mut Vec<Vec<char>>) -> usize {
	let mut bs = [board.clone(), board.clone()];
	let mut ans = 0;
	let mut cur = 0;
	let dirs = [
		[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]
	];
	let (n, m) = (board.len(), board[0].len());
	loop {
		let mut changed = false;
		let nxt = cur ^ 1;
		for i in 0..n {
			for j in 0..m {
				match bs[cur][i][j] {
					'.' => {
						bs[nxt][i][j] = '.';
					}
					'L' => {
						let mut f = false;
						for d in &dirs {
							let mut nx = i as i32;
							let mut ny = j as i32;
							loop {
								nx += d[0];
								ny += d[1];
								if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
									break;
								}
								if bs[cur][nx as usize][ny as usize] == '#' {
									f = true;
									break;
								} else if bs[cur][nx as usize][ny as usize] == 'L' {
									break;
								}
							}
						}
						if !f {
							bs[nxt][i][j] = '#';
							changed = true;
						} else {
							bs[nxt][i][j] = 'L';
						}
					}
					'#' => {
						let mut c = 0;
						for d in &dirs {
							let mut nx = i as i32;
							let mut ny = j as i32;
							loop {
								nx += d[0];
								ny += d[1];
								if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
									break;
								}
								if bs[cur][nx as usize][ny as usize] == '#' {
									c += 1;
									break;
								} else if bs[cur][nx as usize][ny as usize] == 'L' {
									break;
								}
							}
						}
						if c >= 5 {
							bs[nxt][i][j] = 'L';
							changed = true;
						} else {
							bs[nxt][i][j] = '#';
						}
					}
					_ => unreachable!()
				}
			}
		}
		if !changed {
			break;
		}
		cur ^= 1;
	}
	for i in 0..n {
		for j in 0..m {
			if bs[cur][i][j] == '#' {
				ans += 1;
			}
		}
	}
	ans
}
