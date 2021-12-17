#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let points = [[2, 1], [2, 2], [3, 3]];
        let points = points
            .to_vec()
            .into_iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        let angle = 90;
        let location = vec![1, 1];
        let ans = 3;
        assert_eq!(Solution::visible_points(points, angle, location), ans);
    }

    #[test]
    fn test1() {
        let points = [[0, 0], [0, 2]];
        let points = points
            .to_vec()
            .into_iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        let angle = 90;
        let location = vec![1, 1];
        let ans = 2;
        assert_eq!(Solution::visible_points(points, angle, location), ans);
    }

    #[test]
    fn test2() {
        let points = [[2, 2], [3, 3]];
        let points = points
            .to_vec()
            .into_iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        let angle = 0;
        let location = vec![1, 1];
        let ans = 2;
        assert_eq!(Solution::visible_points(points, angle, location), ans);
    }
}

struct Solution {}

use std::f64::consts::PI;

impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut angles = points
            .into_iter()
            .filter_map(|p| {
                let (x, y) = (p[0] - location[0], p[1] - location[1]);
                if x == 0 && y == 0 {
                    ans += 1;
                    None
                } else {
                    Some((y as f64).atan2(x as f64) + PI)
                }
            })
            .collect::<Vec<_>>();
        angles.sort_unstable_by(|a, b| a.partial_cmp(&b).unwrap());
        angles.extend(angles.clone().into_iter().map(|ang| ang + PI * 2.0));
        let angle = (angle as f64).to_radians();
        const EPS: f64 = 1e-10;
        let mut i = 0;
        let mut cnt = 0;
        for j in 0..angles.len() {
            while angles[j] - angles[i] - angle > EPS {
                i += 1;
            }
            cnt = cnt.max(j - i + 1)
        }
        ans + cnt as i32
    }
}
