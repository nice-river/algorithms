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
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut mini = 0i64;
        let mut maxi = 0i64;
        let mut v = 0;
        for diff in differences {
            v += diff as i64;
            mini = mini.min(v);
            maxi = maxi.max(v);
        }
        let lower = lower as i64;
        let upper = upper as i64;
        0.max(upper - (lower - mini + maxi) + 1) as i32
    }
}
