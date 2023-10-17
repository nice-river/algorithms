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

use crate::*;

struct Solution {}

impl Solution {
    pub fn dist_money(mut money: i32, mut children: i32) -> i32 {
        if money < children {
            return -1;
        }
        if money == 4 && children == 1 {
            return -1;
        }
        let mut ans = 0;
        while children > 0 {
            if money - 8 < children - 1 {
                break;
            }
            if money == 12 && children == 2 {
                break;
            }
            if money != 8 && children == 1 {
                break;
            }
            money -= 8;
            children -= 1;
            ans += 1;
        }
        ans
    }
}
