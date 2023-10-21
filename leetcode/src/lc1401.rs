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
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        if x_center < x1 - radius
            || x_center > x2 + radius
            || y_center < y1 - radius
            || y_center > y2 + radius
        {
            return false;
        }
        if x_center < x1 {
            if y_center > y2 && (x_center - x1) * (x_center - x1) + (y_center - y2) * (y_center - y2) > radius * radius {
                return false;
            }
            if y_center < y1 && (x_center - x1) * (x_center - x1) + (y_center - y1) * (y_center - y1) > radius * radius {
                return false;
            }
        } 
        if x_center > x2 {
            if y_center > y2 && (x_center - x2) * (x_center - x2) + (y_center - y2) * (y_center - y2) > radius * radius {
                return false;
            }
            if y_center < y1 && (x_center - x2) * (x_center - x2) + (y_center - y1) * (y_center - y1) > radius * radius {
                return false;
            }

        }
        true
    }
}
