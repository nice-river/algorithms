#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let intervals = to_vec2d([[1, 4], [0, 4]]);
        let ans = to_vec2d([[0, 4]]);
        assert_eq!(Solution::merge(intervals), ans);
    }

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
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        intervals.sort_by_key(|v| {
            (v[0], v[1])
        });
        let mut p = intervals[0].clone();
        for interval in intervals.into_iter().skip(1) {
            if interval[0] > p[1] {
                ans.push(p.clone());
                p = interval.clone();
            } else {
                p[1] = p[1].max(interval[1]);
            }
        }
        ans.push(p);
        ans
    }
}