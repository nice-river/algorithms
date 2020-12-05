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
use std::collections::{HashMap, HashSet};

fn solution() -> std::io::Result<()> {
	let f = File::open("input.txt")?;
	let mut reader = BufReader::new(f);
	let mut line = String::new();
	let mut passport = String::new();
	let mut ans = 0;
	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			if helper(passport.clone()) {
				ans += 1;
			}
			break;
		}
		let s = line.trim();
		if s.is_empty() {
			if helper(passport.clone()) {
				ans += 1;
			}
			passport.clear();
		} else {
			if passport.is_empty() {
				passport = s.to_string();
			} else {
				passport = format!("{} {}", passport, s);
			}
		}

		line.clear();
	}
	println!("{}", ans);
	Ok(())
}

fn helper(passport: String) -> bool {
	let mut m = HashMap::new();
	for part in passport.split(" ") {
		let mut sp = part.split(":");
		let k = sp.next().unwrap();
		let v = sp.next().unwrap();
		m.insert(k, v);
	}
	dbg!(m.clone());
	for (i, &field) in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().enumerate() {
		if !m.contains_key(&field) {
			return false;
		}
		let v = m.get(field).unwrap().clone();
		if i == 0 {
			if v.len() != 4 {
				return false;
			}
			if let Ok(v) = v.parse::<i32>() {
				if v < 1920 || v > 2002 {
					return false;
				}
			} else {
				return false;
			}
		}
		if i == 1 {
			if v.len() != 4 {
				return false;
			}
			if let Ok(v) = v.parse::<i32>() {
				if v < 2010 || v > 2020 {
					return false;
				}
			} else {
				return false;
			}
		}
		if i == 2 {
			if v.len() != 4 {
				return false;
			}
			if let Ok(v) = v.parse::<i32>() {
				if v < 2020 || v > 2030 {
					return false;
				}
			} else {
				return false;
			}
		}
		if i == 3 {
			let v = v.as_bytes();
			let l;
			let r;
			if &v[v.len()-2..] == b"cm" {
				l = 150;
				r = 193;
			} else if &v[v.len()-2..] == b"in" {
				l = 59;
				r = 76;
			} else {
				return false;
			}
			if let Ok(v) = std::str::from_utf8(&v[..v.len()-2]).unwrap().parse::<i32>() {
				if v < l || v > r {
					return false;
				}
			} else {
				return false;
			}
		}
		if i == 4 {
			let v = v.as_bytes();
			if v[0] != b'#' || v.len() != 7 {
				return false;
			}
			for i in 1..7 {
				if !(b'0' <= v[i] && v[i] <= b'9' || b'a' <= v[i] && v[i] <= b'f') {
					return false;
				}
			}
		}
		if i == 5 {
			let cols = "amb blu brn gry grn hzl oth".split(" ").collect::<Vec<&str>>();
			if !cols.contains(&v) {
				return false;
			}
		}
		if i == 6 {
			if v.len() != 9 {
				return false;
			}
			if let Err(_) = v.parse::<usize>() {
				return false;
			}
		}
	}
	true
}