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

use std::collections::HashSet;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut ans = HashSet::new();
        if x == 1 && y == 1 {
            return if bound < 2 { vec![] } else { vec![2] };
        }
        if x == 1 {
            for i in 0.. {
                if y.pow(i) + 1 <= bound {
                    ans.insert(y.pow(i) + 1);
                } else {
                    break;
                }
            }
            return ans.into_iter().collect();
        }
        if y == 1 {
            for i in 0.. {
                if x.pow(i) + 1 <= bound {
                    ans.insert(x.pow(i) + 1);
                } else {
                    break;
                }
            }
            return ans.into_iter().collect();
        }

        let mut a = 0;
        let mut k = 1;
        while k <= bound {
            k *= x;
            a += 1;
        }
        let mut b = 0;
        k = 1;
        while k <= bound {
            k *= y;
            b += 1;
        }
        for i in 0..a {
            for j in 0..b {
                let t = x.pow(i) + y.pow(j);
                if t <= bound {
                    ans.insert(t);
                }
            }
        }
        ans.into_iter().collect()
    }
}
