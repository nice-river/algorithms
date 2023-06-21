#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums1 = vec![13, 67, 90, 92, 47];
        let nums2 = vec![52, 60, 69, 49, 73];
        let queries = vec![vec![32, 70]];
        let ans = vec![120];
        assert_eq!(Solution::maximum_sum_queries(nums1, nums2, queries), ans);
    }

    #[test]
    fn test1() {
        let nums1 = vec![4, 3, 1, 2];
        let nums2 = vec![2, 4, 9, 5];
        let queries = vec![vec![4, 1], vec![1, 3], vec![2, 5]];
        let ans = vec![6, 10, 7];
        assert_eq!(Solution::maximum_sum_queries(nums1, nums2, queries), ans);
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
    pub fn maximum_sum_queries(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut num = nums1
            .into_iter()
            .zip(nums2.into_iter())
            .map(|(a, b)| (a, b))
            .collect::<Vec<_>>();
        num.sort_unstable();
        let mut stk = vec![];
        for (a, b) in num {
            while let Some(&(x, y)) = stk.last() {
                if a < x || b < y {
                    break;
                }
                stk.pop();
            }
            stk.push((a, b));
        }
        let mut ans = vec![-1; queries.len()];
        let mut queries = queries
            .into_iter()
            .enumerate()
            .map(|(i, q)| (q, i))
            .collect::<Vec<_>>();
        queries.sort_by_key(|arr| (arr.0[0], arr.0[1]));
        let mut h = vec![];
        let mut k = stk.len() - 1;
        h.push((stk[k].1, stk[k].0 + stk[k].1));
        while !queries.is_empty() {
            let query = queries.pop().unwrap();
            let a = query.0[0];
            let b = query.0[1];
            if a > stk[k].0 {
                ans[query.1] = -1;
            } else {
                while k > 0 && stk[k - 1].0 >= a {
                    while let Some(&(_, t)) = h.last() {
                        if stk[k - 1].0 + stk[k - 1].1 < t {
                            break;
                        }
                        h.pop();
                    }
                    h.push((stk[k - 1].1, stk[k - 1].0 + stk[k - 1].1));
                    k -= 1;
                }
                let mut l = 0;
                let mut r = h.len();
                while l < r {
                    let m = (l + r) / 2;
                    if h[m].0 < b {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                if r == h.len() {
                    ans[query.1] = -1;
                } else {
                    ans[query.1] = h[r].1;
                }
            }
        }
        ans
    }
}
