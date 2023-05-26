#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ranges = to_vec2d([[34,56],[28,29],[12,16],[11,48],[28,54],[22,55],[28,41],[41,44]]);
        let ans = 2;
        assert_eq!(Solution::count_ways(ranges), ans);
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
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        ranges.sort_by_key(|v| (v[0], v[1]));
        let mut cnt = 1;
        let mut right = ranges[0][1];
        for range in ranges.iter().skip(1) {
            if range[0] > right {
                cnt += 1;
            }
            right = range[1].max(right);
        }
        let mut ans = 1i64;
        for _ in 0..cnt {
            ans = ans * 2 % 1000000007;
        }
        ans as i32
    }
}
