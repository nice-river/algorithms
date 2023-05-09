#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let distance = vec![3, 3, 1, 3];
        assert!(Solution::is_self_crossing(distance));
    }

    #[test]
    fn test1() {
        let distance = vec![1, 1, 2, 1, 1];
        assert!(Solution::is_self_crossing(distance));
    }
}

struct Solution {}

impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let (mut x, mut y) = (0, 0);
        let mut lines = Vec::with_capacity(distance.len());
        static DIRECTIONS: [i32; 5] = [0, -1, 0, 1, 0];
        let mut d = 0;
        for dist in distance {
            let nx = x + dist * DIRECTIONS[d];
            let ny = y + dist * DIRECTIONS[d + 1];
            let new_line = (x.min(nx), y.min(ny), x.max(nx), y.max(ny));
            for line in lines.iter().rev().take(5).skip(1) {
                if Solution::is_intersect(line, &new_line) {
                    return true;
                }
            }
            x = nx;
            y = ny;
            lines.push(new_line);
            d = (d + 1) % 4;
        }
        false
    }

    fn is_intersect(p: &(i32, i32, i32, i32), q: &(i32, i32, i32, i32)) -> bool {
        if q.0 == q.2 {
            if p.0 == p.2 {
                if q.0 != p.0 {
                    false
                } else {
                    (q.1 <= p.1 && p.1 <= q.3) || (q.1 <= p.3 && p.3 <= q.3)
                }
            } else {
                p.0 <= q.0 && q.0 <= p.2 && q.1 <= p.1 && p.1 <= q.3
            }
        } else {
            if p.1 == p.3 {
                if q.1 != p.1 {
                    false
                } else {
                    (q.0 <= p.0 && p.0 <= q.2) || (q.0 <= p.2 && p.2 <= q.2)
                }
            } else {
                q.0 <= p.0 && p.0 <= q.2 && p.1 <= q.1 && q.1 <= p.3
            }
        }
    }
}
