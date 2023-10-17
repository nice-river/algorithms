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
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![];
        for flower in flowers {
            arr.push((flower[0], -1, 0));
            arr.push((flower[1], 1, 0));
        }
        let mut ans = vec![0; people.len()];
        for (i, p) in people.into_iter().enumerate() {
            arr.push((p, 0, i));
        }
        arr.sort();
        let mut n = 0;
        for (_, f, i) in arr {
            if f == -1 {
                n += 1;
            } else if f == 0 {
                ans[i] = n;
            } else {
                n -= 1;
            }
        }
        ans
    }
}
