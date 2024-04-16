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
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        if a == e && (c != a || d < b.min(f) || d > b.max(f)) {
            return 1;
        }
        if b == f && (d != b || c < a.min(e) || c > a.max(e)) {
            return 1;
        }
        for (p, q) in [(1, 1), (-1, 1), (-1, -1), (1, -1)] {
            let mut x = c;
            let mut y = d;
            loop {
                x += p;
                y += q;
                if x < 1 || x > 8 || y < 1 || y > 8 {
                    break;
                }
                if x == e && y == f {
                    return 1;
                }
                if x == a && y == b {
                    break;
                }
            }
        }
        2
    }
}