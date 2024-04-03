#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        // let points = to_vec2d([[3, 10], [5, 15], [10, 2], [4, 4]]);
        // let ans = 12;
        // assert_eq!(Solution::minimum_distance(points), ans);

        let points = to_vec2d([[1, 1], [1, 1], [1, 1]]);
        let ans = 0;
        assert_eq!(Solution::minimum_distance(points), ans);
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

use crate::*;

struct Solution {}

impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let points = points.into_iter().enumerate().collect::<Vec<_>>();
        let edges = Self::find_edge(&points, points.len());
        let mut ans = i32::MAX;
        for edge in edges {
            let cand = Self::find_edge(&points, edge.0);
            ans = ans.min(Self::calc_max_dist(&cand));
        }
        ans
    }

    fn find_edge(points: &Vec<(usize, Vec<i32>)>, ignore_idx: usize) -> Vec<(usize, Vec<i32>)> {
        let mut lb = (i32::MAX, vec![]);
        let mut lt = (i32::MIN, vec![]);
        let mut rb = (i32::MAX, vec![]);
        let mut rt = (i32::MIN, vec![]);

        for point in points {
            if point.0 == ignore_idx {
                continue;
            }
            let x = point.1[0];
            let y = point.1[1];
            if x + y < lb.0 {
                lb.0 = x + y;
                lb.1 = vec![point.clone()];
            } else if x + y == lb.0 {
                lb.1.push(point.clone());
            }
            if x + y > rt.0 {
                rt.0 = x + y;
                rt.1 = vec![point.clone()];
            } else if x + y == rt.0 {
                rt.1.push(point.clone());
            }
            if y - x < rb.0 {
                rb.0 = y - x;
                rb.1 = vec![point.clone()];
            } else if y - x == rb.0 {
                rb.1.push(point.clone());
            }
            if y - x > lt.0 {
                lt.0 = y - x;
                lt.1 = vec![point.clone()];
            } else if y - x == lt.0 {
                lt.1.push(point.clone());
            }
        }
        let mut ret = vec![];
        for mut arr in [lb, rb, rt, lt] {
            arr.1.sort_by_key(|(_, v)| (v[0], v[1]));
            if arr.1.len() == 1 {
                ret.push(arr.1[0].clone());
            } else {
                ret.push(arr.1[0].clone());
                ret.push(arr.1.last().unwrap().clone());
            }
        }
        ret
    }

    fn calc_max_dist(points: &Vec<(usize, Vec<i32>)>) -> i32 {
        let mut t = 0;
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                t = t.max(
                    (points[i].1[0] - points[j].1[0]).abs()
                        + (points[i].1[1] - points[j].1[1]).abs(),
                );
            }
        }
        t
    }
}
