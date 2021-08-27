struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let buildings = vec![vec![0,2,3],vec![2,5,3]];
		let ans = vec![vec![0, 3], vec![5, 0]];
		assert_eq!(Solution::get_skyline(buildings), ans);
	}

	#[test]
	fn test1() {
		let buildings = vec![vec![2,9,10],vec![3,7,15],vec![5,12,12],vec![15,20,10],vec![19,24,8]];
		let ans = vec![vec![2,10],vec![3,15],vec![7,12],vec![12,0],vec![15,10],vec![20,8],vec![24,0]];

		assert_eq!(Solution::get_skyline(buildings), ans);
	}

	#[test]
	fn test2() {
		let buildings = vec![vec![2,9,10]];
		let ans = vec![vec![2, 10], vec![9, 0]];

		assert_eq!(Solution::get_skyline(buildings), ans);
	}
}

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};


#[derive(Clone, Debug, Eq, PartialEq)]
struct Line {
	axis: i32,
	height: i32,
	is_left: bool,
    idx: usize,
}

impl PartialOrd for Line {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.axis != other.axis {
			return self.axis.partial_cmp(&other.axis);
		}
        if self.is_left && !other.is_left {
			return Some(Ordering::Less);
		}
        if !self.is_left && other.is_left {
			return Some(Ordering::Greater);
		}
        match self.height.cmp(&other.height) {
			Ordering::Less => Some(Ordering::Greater),
			Ordering::Equal => Some(Ordering::Equal),
			Ordering::Greater => Some(Ordering::Less),
		}
	}
}

impl Ord for Line {
	fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
	}
}


impl Line {
    fn new(idx: usize, axis: i32, height: i32, is_left: bool) -> Self {
        Self {
			axis, height, is_left, idx
		}
	}
}


impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut lines = vec![];
		for (idx, building) in buildings.into_iter().enumerate() {
            lines.push(Line::new(idx, building[0], building[2], true));
			lines.push(Line::new(idx, building[1], building[2], false));
		}
		lines.sort_unstable();

		let mut map = BTreeMap::new();
		let mut ans = vec![];

		let mut i = 0;
        while i < lines.len() {
            if lines[i].is_left {
				if let Some((&max_height, _)) = map.iter().rev().next() {
                    if lines[i].height > max_height {
                        ans.push(vec![lines[i].axis, lines[i].height]);
					}
				} else {
                    ans.push(vec![lines[i].axis, lines[i].height]);
				}
                map.entry(lines[i].height).or_insert(HashSet::new()).insert(lines[i].idx);
				i += 1;
			} else {
                let mut j = i;
				while j < lines.len() && lines[j].axis == lines[i].axis {
                    map.get_mut(&lines[j].height).unwrap().remove(&lines[j].idx);
                    if map.get(&lines[j].height).unwrap().len() == 0 {
						map.remove(&lines[j].height);
					}
					j += 1;
				}
				if let Some((&max_height, _)) = map.iter().rev().next() {
                    if max_height < ans.last().unwrap()[1] {
						ans.push(vec![lines[i].axis, max_height])
					}
				} else if j != lines.len() {
					ans.push(vec![lines[i].axis, 0])
				}
				i = j;
			}
		}
        ans.push(vec![lines.last().unwrap().axis, 0]);
		ans
	}
}