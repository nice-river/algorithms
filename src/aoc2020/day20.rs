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
use std::process::id;
use std::intrinsics::transmute;

const N: usize = 10;
const PATTERN_IDXES: [[i32; 2]; 15] = [[0, 0], [1, 1], [1, 4], [0, 5], [0, 6], [1, 7], [1, 10], [0, 11], [0, 12], [1, 13], [1, 16], [0, 17], [-1, 18], [0, 18], [0, 19]];

fn transpose(matrix: &mut Vec<Vec<u8>>) {
	let n = matrix.len();
	if n == 21 {
		println!();
	}
	for i in 1..n {
		for j in 0..i {
			let t = matrix[i][j];
			matrix[i][j] = matrix[j][i];
			matrix[j][i] = t;
		}
	}
}

fn back_transpose(matrix: &mut Vec<Vec<u8>>) {
	let n = matrix.len();
	for i in 1..n {
		for j in 0..n-1-i {
			let t = matrix[i][j];
			matrix[i][j] = matrix[n-1-j][n-1-i];
			matrix[n-1-j][n-1-i] = t;
		}
	}
}

fn flip_h(matrix: &mut Vec<Vec<u8>>) {
	let n = matrix.len();
	for i in 0..n {
		for j in 0..n/2 {
			let t = matrix[i][j];
			matrix[i][j] = matrix[i][n-1-j];
			matrix[i][n-1-j] = t;
		}
	}
}

fn flip_v(matrix: &mut Vec<Vec<u8>>) {
	let n = matrix.len();
	for i in  0..n/2 {
		for j in 0..n {
			let t = matrix[i][j];
			matrix[i][j] = matrix[n-1-i][j];
			matrix[n-1-i][j] = t;
		}
	}
}

struct Tile {
	id: usize,
	tiles: Vec<Vec<Vec<u8>>>,
	tile_idx: usize,
}

impl Tile {
	fn new(mut vs: Vec<String>) -> Self {
		let tile_id = vs.first().unwrap().split(" ").nth(1).unwrap();
		let tile_id = tile_id[0..tile_id.len()-1].parse::<usize>().unwrap();
		vs.remove(0);
		let vs: Vec<Vec<u8>>  = vs.into_iter().map(|s| s.into_bytes()).collect();
		let mut tiles = vec![vs.clone()];
		for _ in 0..3 {
			let mut tile = tiles.last().unwrap().clone();
			transpose(&mut tile);
			flip_h(&mut tile);
			tiles.push(tile);
		}
		let mut tile = vs.clone();
		flip_h(&mut tile);
		tiles.push(tile);
		let mut tile = vs.clone();
		flip_v(&mut tile);
		tiles.push(tile);
		let mut tile = vs.clone();
		transpose(&mut tile);
		tiles.push(tile);
		let mut tile = vs.clone();
		back_transpose(&mut tile);
		tiles.push(tile);

		Self {
			id: tile_id,
			tiles,
			tile_idx: 0,
		}
	}
}

fn solution() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();

	let mut tile: Vec<String> = vec![];
	let mut tiles = vec![];


	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}
		let s = line.trim().to_string();
		if s.is_empty() && !tile.is_empty(){
			tiles.push(Tile::new(tile.clone()));
			tile.clear();
		} else {
			tile.push(s);
		}
		line.clear();
	}
	if!tile.is_empty() {
		tiles.push(Tile::new(tile));
	}
	let n = tiles.len();
	let mut used = vec![false; n];
	let mut idxes = vec![];
	let n = (n as f64).sqrt() as usize;
	helper(&mut idxes, &mut tiles, &mut used, n);
	// let f = vec![0, n-1, n * (n - 1), n * n - 1];
	// let ans = f.into_iter().fold(1, |acc, k| {
	// 	acc * tiles[idxes[k]].id
	// });
	println!("{:?}", helper2(&idxes, &tiles));
	Ok(())
}

fn helper(idxes: &mut Vec<usize>, tiles: &mut Vec<Tile>, used: &mut Vec<bool>, n: usize) -> bool {
	if idxes.len() == tiles.len() {
		return true;
	}

	for i in 0..used.len() {
		if used[i] {
			continue ;
		}
		let tile = tiles.get(i).unwrap();
		let row = idxes.len() / n;
		let col = idxes.len() % n;
		let mut set: HashSet<usize> = HashSet::from_iter((0..tile.tiles.len()));
		if row != 0 {
			let pre = &tiles[idxes[idxes.len() - n]];
			let pre = &pre.tiles[pre.tile_idx];
			'find_row_tile: for (c, k) in tile.tiles.iter().enumerate() {
				for j in 0..N {
					if pre[N-1][j] != k[0][j] {
						set.remove(&c);
						continue 'find_row_tile;
					}
				}
			}
		}
		if col != 0 {
			let pre = &tiles[idxes[idxes.len() - 1]];
			let pre = &pre.tiles[pre.tile_idx];
			'find_col_tile: for (c, k) in tile.tiles.iter().enumerate() {
				for j in 0..N {
					if pre[j][N-1] != k[j][0] {
						set.remove(&c);
						continue 'find_col_tile;
					}
				}
			}
		}
		for &idx in set.iter() {
			used[i] = true;
			idxes.push(i);
			tiles.get_mut(i).unwrap().tile_idx = idx;
			if helper(idxes, tiles, used, n) {
				return true;
			}
			idxes.pop();
			used[i] = false;
		}
	}
	false
}

fn helper2(idxes: &Vec<usize>, tiles: &Vec<Tile>) -> i32 {
	let origin_image = construct_image(idxes, tiles);
	let mut image = origin_image.clone();
	for _ in 0..4 {
		let xys = get_pattern_axis(&image);
		if !xys.is_empty() {
			return get_sea_num(&xys, &image);
		}
		transpose(&mut image);
		flip_h(&mut image);
	}

	let mut image = origin_image.clone();
	transpose(&mut image);
	let xys = get_pattern_axis(&image);
	if !xys.is_empty() {
		return get_sea_num(&xys, &image);
	}

	let mut image = origin_image.clone();
	flip_h(&mut image);
	let xys = get_pattern_axis(&image);
	if !xys.is_empty() {
		return get_sea_num(&xys, &image);
	}

	let mut image = origin_image.clone();
	flip_v(&mut image);
	let xys = get_pattern_axis(&image);
	if !xys.is_empty() {
		return get_sea_num(&xys, &image);
	}

	let mut image = origin_image.clone();
	back_transpose(&mut image);
	let xys = get_pattern_axis(&image);
	if !xys.is_empty() {
		return get_sea_num(&xys, &image);
	}

	0
}

fn construct_image(idxes: &Vec<usize>, tiles: &Vec<Tile>) -> Vec<Vec<u8>> {
	let n = tiles.len();
	let n = (n as f64).sqrt() as usize;
	let mut ret = vec![];
	for i in 0..n {
		let mut m = vec![vec![]; N - 2];
		for j in 0..n {
			let idx = idxes[i * n + j];
			let tile = &tiles[idx];
			let tile = &tile.tiles[tile.tile_idx];
			for x in 1..N-1 {
				for y in 1..N-1 {
					m[x-1].push(tile[x][y]);
				}
			}
		}
		ret.extend(m);
	}
	ret
}

fn get_pattern_axis(image: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
	let n = image.len();
	let mut ret = vec![];
	for x in 0..n {
		for y in 0..n {
			if find_pattern(x, y, image) {
				for z in 0..PATTERN_IDXES.len() {
					let i = ((x as i32) + PATTERN_IDXES[z][0]) as usize;
					let j = ((y as i32) + PATTERN_IDXES[z][1]) as usize;
					ret.push((i, j));
				}
			}
		}
	}
	ret
}


fn find_pattern(x: usize, y: usize, image: &Vec<Vec<u8>>) -> bool {
	// .#.#...#.###...#.##.O#..
	// #.O.##.OO#.#.OO.##.OOO##
	// ..#O.#O#.O##O..O.#O##.##
	let n = image.len();
	if x == 0 || x == n - 1 || y + 19 >= n {
		return false;
	}
	for idx in PATTERN_IDXES.iter() {
		let i = ((x as i32) + idx[0]) as usize;
		let j = ((y as i32) + idx[1]) as usize;
		if image[i][j] != b'#' {
			return false;
		}
	}
	true
}

fn get_sea_num(xys: &Vec<(usize, usize)>, image: &Vec<Vec<u8>>) -> i32 {
	let n = image.len();
	let mut ret = 0;
	for i in 0..n {
		for j in 0..n {
			if image[i][j] == b'#' && !xys.contains(&(i, j)) {
				ret += 1;
			}
		}
	}
	ret
}
