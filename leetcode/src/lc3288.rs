#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let coordinates = to_vec!([[8, 4], [8, 9], [5, 2], [2, 4], [3, 2], [3, 1]]);
        let k = 0;
        let ans = 3;
        assert_eq!(Solution::max_path_length(coordinates, k), ans);
    }

    #[test]
    fn test1() {
        let coordinates = to_vec!([[5, 0], [9, 3], [9, 8]]);
        let k = 0;
        let ans = 2;
        assert_eq!(Solution::max_path_length(coordinates, k), ans);
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_path_length(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let base = coordinates[k as usize].clone();
        let mut part = coordinates
            .iter()
            .filter(|&x| x[0] > base[0] && x[1] > base[1])
            .collect::<Vec<_>>();
        part.sort_by_key(|v| (v[0], -v[1]));
        let mut ans = Self::max_seq_length(part.into_iter());
        let mut part = coordinates
            .iter()
            .filter(|&x| x[0] < base[0] && x[1] < base[1])
            .collect::<Vec<_>>();
        part.sort_by_key(|v| (v[0], -v[1]));
        ans += Self::max_seq_length(part.into_iter());
        ans + 1
    }

    fn max_seq_length<'a>(mut arr: impl Iterator<Item = &'a Vec<i32>>) -> i32 {
        if let Some(v) = arr.next() {
            let mut seq = vec![v[1]];
            for v in arr {
                let y = v[1];
                if y > seq[seq.len() - 1] {
                    seq.push(y);
                } else {
                    match seq.binary_search(&y) {
                        Ok(_) => {}
                        Err(idx) => {
                            seq[idx] = y;
                        }
                    }
                }
            }
            seq.len() as i32
        } else {
            0
        }
    }
}
