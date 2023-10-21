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

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        cost.reverse();
        let mut i = 0;
        let mut ans = 0;
        while i < cost.len() {
            ans += cost[i];
            if i + 1 < cost.len() {
                ans += cost[i + 1];
            }
            i += 3;
        }
        ans
    }
}