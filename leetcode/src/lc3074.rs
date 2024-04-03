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
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort();
        capacity.reverse();
        let mut ans = 0;
        let all = apple.into_iter().sum::<i32>();
        let mut x = 0;
        for cap in capacity {
            ans += 1;
            if x + cap >= all {
                return ans;
            }
            x += cap;
        }
        unreachable!()
    }
}
