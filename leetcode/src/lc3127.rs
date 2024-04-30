#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let mut t = vec![0, 0];
                for delta in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    if grid[i + delta.0][j + delta.1] == 'B' {
                        t[0] += 1;
                    } else {
                        t[1] += 1;
                    }
                }
                if t[0] != 2 {
                    return true;
                }
            }
        }
        false
    }
}
