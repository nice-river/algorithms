#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let expr = String::from("12+34");
        let ans = String::from("1(2+3)4");
        assert_eq!(Solution::minimize_result(expr), ans);
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
    pub fn minimize_result(expression: String) -> String {
        let mut expr = expression.into_bytes();
        let add = expr.iter().position(|&c| c == b'+').unwrap();
        let mut mini = i32::MAX;
        let mut ans = (0, expr.len());
        for i in 0..add {
            for j in add + 2..=expr.len() {
                let a = std::str::from_utf8(&expr[0..i])
                    .unwrap()
                    .parse::<i32>()
                    .unwrap_or(1);
                let b = std::str::from_utf8(&expr[i..add])
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let c = std::str::from_utf8(&expr[add + 1..j])
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let d = std::str::from_utf8(&expr[j..expr.len()])
                    .unwrap()
                    .parse::<i32>()
                    .unwrap_or(1);
                if a * (b + c) * d < mini {
                    mini = a * (b + c) * d;
                    ans = (i, j);
                }
            }
        }
        expr.insert(ans.1, b')');
        expr.insert(ans.0, b'(');
        String::from_utf8(expr).unwrap()
    }
}
