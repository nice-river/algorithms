#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let (ax0, ax1, ay0, ay1) = (-2, -2, 2, 2);
        let (bx0, bx1, by0, by1) = (-2, -2, 2, 2);
        let ans = 16;
        assert_eq!(
            Solution::compute_area(ax0, ax1, ay0, ay1, bx0, bx1, by0, by1),
            ans
        );
    }
}

struct Solution {}

struct Rect {
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
}

impl Rect {
    fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Self { x0, x1, y0, y1 }
    }

    fn get_area(&self) -> i32 {
        (self.x1 - self.x0) * (self.y1 - self.y0)
    }

    fn get_intersect_area(&self, oth: &Rect) -> i32 {
        if oth.x0 >= self.x1 || oth.x1 <= self.x0 {
            return 0;
        }
        if oth.y0 >= self.y1 || oth.y1 <= self.y0 {
            return 0;
        }
        (self.x1.min(oth.x1) - self.x0.max(oth.x0)) * (self.y1.min(oth.y1) - self.y0.max(oth.y0))
    }
}

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let a = Rect::new(ax1, ay1, ax2, ay2);
        let b = Rect::new(bx1, by1, bx2, by2);
        let mut ans = a.get_area() + b.get_area();
        ans -= a.get_intersect_area(&b);
        ans
    }
}
