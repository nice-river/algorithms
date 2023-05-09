#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let numerator = 2;
        let denominator = 3;
        let ans = "0.(6)".to_string();
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), ans);
    }

    #[test]
    fn test1() {
        let numerator = 1;
        let denominator = -6;
        let ans = "-0.1(6)".to_string();
        assert_eq!(Solution::fraction_to_decimal(numerator, denominator), ans);
    }
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let numerator = numerator as i64;
        let denominator = denominator as i64;
        let is_less_zero = if numerator * denominator < 0 {
            true
        } else {
            false
        };
        let mut numerator = numerator.abs();
        let denominator = denominator.abs();
        let mut map = HashMap::new();
        let mut a = numerator / denominator;
        let mut b = vec![];
        numerator %= denominator;
        numerator *= 10;
        while numerator != 0 {
            if let Some(&p) = map.get(&numerator) {
                b.insert(p, "(".to_string());
                b.push(")".to_string());
                break;
            }
            map.insert(numerator, b.len());
            if numerator < denominator {
                b.push("0".to_string());
                numerator *= 10;
            } else {
                b.push((numerator / denominator).to_string());
                numerator %= denominator;
                numerator *= 10;
            }
        }
        let s = if b.is_empty() {
            a.to_string()
        } else {
            format!("{}.{}", a, b.join(""))
        };
        if is_less_zero {
            format!("-{}", s)
        } else {
            s
        }
    }
}
