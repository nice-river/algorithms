#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let num = "2".to_owned();
        let t = 12;
        let ans = "26";
        assert_eq!(Solution::smallest_number(num, t), ans);
    }

    #[test]
    fn test1() {
        let num = "1234".to_owned();
        let t = 256;
        let ans = "1488";
        assert_eq!(Solution::smallest_number(num, t), ans);
    }

    #[test]
    fn test2() {
        let num = "12355".to_owned();
        let t = 50;
        let ans = "12355";
        assert_eq!(Solution::smallest_number(num, t), ans);
    }

    #[test]
    fn test3() {
        let num = "11111".to_owned();
        let t = 26;
        let ans = "-1";
        assert_eq!(Solution::smallest_number(num, t), ans);
    }
}

use crate::*;

struct Solution {}

use std::{collections::HashMap, mem::discriminant};

#[derive(Debug)]
struct Helper {
    freq: HashMap<i32, i32>,
}

impl Helper {
    fn new(freq: HashMap<i32, i32>) -> Self {
        Self { freq }
    }

    fn min_digit_num(&self) -> usize {
        let mut twos = *self.freq.get(&2).unwrap_or(&0);
        let threes = *self.freq.get(&3).unwrap_or(&0);
        let a = twos / 3;
        twos %= 3;
        let b = (threes + twos + 1) / 2;
        (a + b + *self.freq.get(&5).unwrap_or(&0) + *self.freq.get(&7).unwrap_or(&0)) as usize
    }

    fn gen_min_str(&self) -> Vec<u8> {
        let mut ret = vec![];
        let mut twos = *self.freq.get(&2).unwrap_or(&0);
        let threes = *self.freq.get(&3).unwrap_or(&0);
        let a = twos / 3;
        twos %= 3;
        ret.extend(std::iter::repeat(b'8').take(a as usize));
        ret.extend(std::iter::repeat(b'9').take(threes as usize / 2));
        match threes % 2 {
            0 => {
                if twos == 1 {
                    ret.push(b'2');
                } else if twos == 2 {
                    ret.push(b'4');
                }
            }
            1 => {
                if twos == 1 {
                    ret.push(b'6');
                } else if twos == 2 {
                    ret.push(b'6');
                    ret.push(b'2');
                } else if twos == 0 {
                    ret.push(b'3');
                }
            }
            _ => unreachable!(),
        }
        ret.extend(std::iter::repeat(b'5').take(*self.freq.get(&5).unwrap_or(&0) as usize));
        ret.extend(std::iter::repeat(b'7').take(*self.freq.get(&7).unwrap_or(&0) as usize));
        ret.sort();
        ret
    }

    fn pop(&mut self, digit: u8) -> Option<u8> {
        match digit {
            b'1' => {
                return Some(b'1');
            }
            b'2' => {
                if let Some(x) = self.freq.get_mut(&2) {
                    if *x >= 1 {
                        *x -= 1;
                        return Some(b'2');
                    }
                }
            }
            b'3' => {
                if let Some(x) = self.freq.get_mut(&3) {
                    if *x >= 1 {
                        *x -= 1;
                        return Some(b'3');
                    }
                }
            }
            b'4' => {
                if let Some(x) = self.freq.get_mut(&2) {
                    if *x >= 2 {
                        *x -= 2;
                        return Some(b'4');
                    }
                }
            }
            b'5' => {
                if let Some(x) = self.freq.get_mut(&5) {
                    if *x >= 1 {
                        *x -= 1;
                        return Some(b'5');
                    }
                }
            }
            b'6' => {
                let x = *self.freq.get(&2).unwrap_or(&0);
                let y = *self.freq.get(&3).unwrap_or(&0);
                if x > 0 && y > 0 {
                    *self.freq.get_mut(&2).unwrap() -= 1;
                    *self.freq.get_mut(&3).unwrap() -= 1;
                    return Some(b'6');
                }
            }
            b'7' => {
                if let Some(x) = self.freq.get_mut(&7) {
                    if *x >= 1 {
                        *x -= 1;
                        return Some(b'7');
                    }
                }
            }
            b'8' => {
                if let Some(x) = self.freq.get_mut(&2) {
                    if *x >= 3 {
                        *x -= 3;
                        return Some(b'8');
                    }
                }
            }
            b'9' => {
                if let Some(x) = self.freq.get_mut(&3) {
                    if *x >= 2 {
                        *x -= 2;
                        return Some(b'9');
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ => unreachable!(),
        }
        self.pop(digit + 1)
    }

    fn push(&mut self, digit: u8) {
        match digit {
            b'2' => {
                *self.freq.entry(2).or_insert(0) += 1;
            }
            b'3' => {
                *self.freq.entry(3).or_insert(0) += 1;
            }
            b'4' => {
                *self.freq.entry(2).or_insert(0) += 2;
            }
            b'5' => {
                *self.freq.entry(5).or_insert(0) += 1;
            }
            b'6' => {
                *self.freq.entry(2).or_insert(0) += 1;
                *self.freq.entry(3).or_insert(0) += 1;
            }
            b'7' => {
                *self.freq.entry(7).or_insert(0) += 1;
            }
            b'8' => {
                *self.freq.entry(2).or_insert(0) += 3;
            }
            b'9' => {
                *self.freq.entry(3).or_insert(0) += 2;
            }
            _ => unreachable!(),
        }
    }
}

impl Solution {
    pub fn smallest_number(num: String, mut t: i64) -> String {
        if t == 1 {
            return num;
        }
        let num = num.as_bytes();
        let mut freq = HashMap::new();
        while t != 1 {
            let mut avail = false;
            for k in [2, 3, 5, 7] {
                if t % k == 0 {
                    avail = true;
                    t /= k;
                    *freq.entry(k as i32).or_insert(0) += 1;
                }
            }
            if !avail {
                return "-1".to_owned();
            }
        }
        let mut helper = Helper::new(freq);
        dbg!(&helper);
        if helper.min_digit_num() > num.len() {
            String::from_utf8(helper.gen_min_str()).unwrap()
        } else {
            let mut ans = Vec::with_capacity(num.len() + 1);
            if Self::dfs(num, &mut ans, 0, false, &mut helper) {
                return String::from_utf8(ans).unwrap();
            }
            ans.clear();
            ans.push(b'1');
            let x = helper.min_digit_num();
            ans.extend(std::iter::repeat(b'1').take(num.len() - x));
            ans.extend(helper.gen_min_str());
            String::from_utf8(helper.gen_min_str()).unwrap()
        }
    }

    fn dfs(num: &[u8], ans: &mut Vec<u8>, idx: usize, bigger: bool, helper: &mut Helper) -> bool {
        dbg!((String::from_utf8(ans.clone()).unwrap(), idx, bigger));
        let x = helper.min_digit_num();
        if x > num.len() - idx {
            return false;
        }
        if bigger {
            if x <= num.len() - idx {
                ans.extend(std::iter::repeat(b'1').take(num.len() - idx - x));
                ans.extend(helper.gen_min_str());
                return true;
            }
            return false;
        }
        let start = if bigger { b'1' } else { num[idx] };
        for c in start..=b'9' {
            if let Some(g) = helper.pop(c) {
                ans.push(g);
                if Solution::dfs(num, ans, idx + 1, bigger || g > num[idx], helper) {
                    return true;
                } else {
                    helper.push(g);
                    ans.pop();
                }
            }
        }
        false
    }
}
