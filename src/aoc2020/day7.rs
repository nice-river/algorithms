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
	let mut index = 0;

	let mut name = HashMap::new();
	let mut cnt = HashMap::new();

	let mut parent = vec![];

	while let Ok(l) = reader.read_line(&mut line) {
		if l == 0 {
			break;
		}

		let r = helper(&line);

		parent.push(r[0].clone().0);

		name.entry(r[0].clone().0).or_insert_with(|| {
			index += 1;
			index
		});

		for i in 1..r.len() {
			name.entry(r[i].clone().0).or_insert_with(|| {
				index += 1;
				index
			});
		}

		cnt.insert(name[&r[0].0], HashMap::new());
		let v = cnt.get_mut(&name[&r[0].0]).unwrap();
		for i in 1..r.len() {
			v.insert(name[&r[i].0], r[i].1 as usize);
		}

		line.clear();
	}

	let mut memo = HashMap::new();
	let base = get(name["shiny gold"], &cnt, &mut memo);

	println!("{:?}", base);
	Ok(())
}

fn get(k: i32, cnt: &HashMap<i32, HashMap<i32, usize>>, memo: &mut HashMap<i32, usize>) -> usize {
	if memo.contains_key(&k) {
		return memo[&k].clone();
	}
	let mut ret = 0;
	let default = HashMap::new();

	for (&nk, &nv) in cnt.get(&k).unwrap() {
		ret += nv;
		if cnt.get(&nk).unwrap_or(&default).len() != 0 {
			let m = get(nk, cnt, memo);
			ret += m * nv;
		}
	}
	*memo.entry(k).or_default() = ret;
	ret
}

fn helper(s: &str) -> Vec<(String, i32)>{
	let mut sp = s.split("contain");
	let p = sp.next().unwrap();
	let q = sp.next().unwrap();
	let mut p = p.split(" ").collect::<Vec<&str>>();
	let mut ret = Vec::new();
	for i in 0..p.len() {
		if p[i].starts_with("bag") {
			ret.push((p[..i].join(" "), 0));
			break;
		}
	}
	let mut q = q.split(",").collect::<Vec<&str>>();
	for s in q {
		let sp = s.trim().split(" ").collect::<Vec<&str>>();
		if let Ok(num) = sp[0].parse::<i32>() {
			for i in 1..sp.len() {
				if sp[i].starts_with("bag") {
					ret.push((sp[1..i].join(" "), num));
					break;
				}
			}
		}
	}
	ret
}