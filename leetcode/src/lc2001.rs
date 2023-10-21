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

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut map = HashMap::new();
        for rect in rectangles {
            let w = rect[0];
            let h = rect[1];
            let g = Self::gcd(w, h);
            *map.entry((w / g, h / g)).or_insert(0) += 1;
        }
        let mut ans = 0;
        for (_, v) in map.into_iter() {
            ans += v * (v - 1) / 2;
        }
        ans
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
