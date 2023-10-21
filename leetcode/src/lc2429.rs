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
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut ones = num2.count_ones();
        let mut ans = 0;
        for i in (0..32).rev() {
            if ones == 0 {
                break;
            }
            if (num1 & (1 << i)) != 0 {
                ans |= 1 << i;
                ones -= 1;
            }
        }
        for i in 0..32 {
            if ones == 0 {
                break;
            }
            if (ans & (1 << i)) == 0 {
                ans |= 1 << i;
                ones -= 1;
            }
        }
        ans
    }
}
