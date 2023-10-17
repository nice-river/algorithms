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
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; prices.len()];
        for i in 0..prices.len() {
            let mut p = prices[i];
            for j in i + 1..prices.len() {
                if prices[j] <= prices[i] {
                    p -= prices[j];
                    break;
                }
            }
            ans[i] = p;
        }
        ans
    }
}
