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
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut cur = vec![vec![b'0'], vec![b'1']];
        let mut nxt = vec![];
        for _ in 2..=n {
            for mut arr in cur {
                if arr.last().unwrap() == &b'1' {
                    let mut g = arr.clone();
                    g.push(b'0');
                    nxt.push(g);
                    arr.push(b'1');
                    nxt.push(arr);
                } else {
                    arr.push(b'1');
                    nxt.push(arr);
                }
            }
            cur = nxt;
            nxt = vec![];
        }

        cur.into_iter().map(|arr| String::from_utf8(arr).unwrap()).collect()
    }
}
