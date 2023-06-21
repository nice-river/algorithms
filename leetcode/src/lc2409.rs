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
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let a = Self::convert(arrive_alice);
        let b = Self::convert(leave_alice);
        let c = Self::convert(arrive_bob);
        let d = Self::convert(leave_bob);
        (b.min(d) - a.max(c) + 1).max(0)
    }

    fn convert(s: String) -> i32 {
        let mut s = s.split('-');
        let a: usize = s.next().unwrap().parse().unwrap();
        let b: i32 = s.next().unwrap().parse().unwrap();
        let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut day = 0;
        for i in 0..12 {
            if i + 1 < a {
                day += days[i];
            }
        }
        day + b
    }
}
