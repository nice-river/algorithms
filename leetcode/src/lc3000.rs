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
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut mini = 0;
        let mut ans = 0;
        for dimensions in dimensions {
            let a = dimensions[0] as i64;
            let b = dimensions[1] as i64;
            let t = a * a + b * b;
            if t > mini {
                ans = a * b;
                mini = t;
            } else if t == mini {
                ans = ans.max(a * b);
            }
        }
        ans as i32
    }
}