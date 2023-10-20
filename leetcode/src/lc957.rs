#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let cells = vec![0, 1, 0, 1, 1, 0, 0, 1];
        let n = 7;
        let ans = vec![0, 0, 1, 1, 0, 0, 0, 0];
        assert_eq!(Solution::prison_after_n_days(cells, n), ans);
    }

    #[test]
    fn test1() {
        let cells = vec![1, 0, 0, 1, 0, 0, 1, 0];
        let n = 1000000000;
        let ans = vec![0, 0, 1, 1, 1, 1, 1, 0];
        assert_eq!(Solution::prison_after_n_days(cells, n), ans);
    }

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let sz = cells.len();
        let mut repeats = vec![];
        repeats.push(cells.clone());
        let mut k = 0;
        while k < n {
            k += 1;
            let mut new_cells = vec![0; cells.len()];
            for i in 1..sz - 1 {
                if cells[i - 1] == cells[i + 1] {
                    new_cells[i] = 1;
                } else {
                    new_cells[i] = 0;
                }
            }
            for (i, arr) in repeats.iter().enumerate() {
                if &new_cells == arr {
                    return repeats[i + (n as usize - i) % (k - i)].clone();
                }
            }
            for i in 0..sz {
                cells[i] = new_cells[i];
            }
            repeats.push(new_cells);
        }
        repeats.last().unwrap().clone()
    }
}
