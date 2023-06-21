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
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ans = vec![vec![0; n]; n];
        let mut ops = vec![vec![]; n];
        for (i, q) in queries.iter().enumerate() {
            ops[q[0] as usize].push((0, i));
            ops[q[2] as usize].push((1, i));
        }
        let mut val = vec![0; n];
        for i in 0..n {
            ops[i].sort_unstable();
            let mut j = 0;
            while j < ops[i].len() && ops[i][j].0 == 0 {
                let q = &queries[ops[i][j].1];
                for k in q[1]..=q[3] {
                    val[k as usize] += 1;
                }
                j += 1;
            }
            ans[i] = val.clone();
            while j < ops[i].len() {
                let q = &queries[ops[i][j].1];
                for k in q[1]..=q[3] {
                    val[k as usize] -= 1;
                }
                j += 1;
            }
        }

        ans
    }
}
