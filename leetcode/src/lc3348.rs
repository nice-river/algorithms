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

    #[test]
    fn test4() {
        let num = "30".to_owned();
        let t = 9;
        let ans = "33";
        assert_eq!(Solution::smallest_number(num, t), ans);
    }

    #[test]
    fn test5() {
        let num = "78".to_owned();
        let t = 42;
        let ans = "167";
        assert_eq!(Solution::smallest_number(num, t), ans);
    }
}

use crate::*;

struct Solution {}

use std::collections::HashMap;

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

    fn pop(&mut self, digit: u8) -> Vec<u8> {
        let mut ret = vec![];
        match digit {
            b'1' => {
                return ret;
            }
            b'2' => {
                if let Some(x) = self.freq.get_mut(&2) {
                    if *x >= 1 {
                        *x -= 1;
                        ret.push(2);
                    }
                }
            }
            b'3' => {
                if let Some(x) = self.freq.get_mut(&3) {
                    if *x >= 1 {
                        *x -= 1;
                        ret.push(3);
                    }
                }
            }
            b'4' => {
                if let Some(x) = self.freq.get_mut(&2) {
                    for _ in 0..2 {
                        if *x >= 1 {
                            *x -= 1;
                            ret.push(2);
                        }
                    }
                }
            }
            b'5' => {
                if let Some(x) = self.freq.get_mut(&5) {
                    if *x >= 1 {
                        *x -= 1;
                        ret.push(5);
                    }
                }
            }
            b'6' => {
                let x = *self.freq.get(&2).unwrap_or(&0);
                let y = *self.freq.get(&3).unwrap_or(&0);
                if x > 0 {
                    *self.freq.get_mut(&2).unwrap() -= 1;
                    ret.push(2);
                }
                if y > 0 {
                    *self.freq.get_mut(&3).unwrap() -= 1;
                    ret.push(3);
                }
            }
            b'7' => {
                if let Some(x) = self.freq.get_mut(&7) {
                    if *x >= 1 {
                        *x -= 1;
                        ret.push(7);
                    }
                }
            }
            b'8' => {
                if let Some(x) = self.freq.get_mut(&2) {
                    for _ in 0..3 {
                        if *x >= 1 {
                            *x -= 1;
                            ret.push(2);
                        }
                    }
                }
            }
            b'9' => {
                if let Some(x) = self.freq.get_mut(&3) {
                    for _ in 0..2 {
                        if *x >= 1 {
                            *x -= 1;
                            ret.push(3);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        ret
    }

    fn push(&mut self, digits: Vec<u8>) {
        for digit in digits {
            *self.freq.entry(digit as i32).or_insert(0) += 1;
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
            String::from_utf8(ans).unwrap()
        }
    }

    fn dfs(num: &[u8], ans: &mut Vec<u8>, idx: usize, bigger: bool, helper: &mut Helper) -> bool {
        let x = helper.min_digit_num();
        if x > num.len() - idx {
            return false;
        }
        if idx == num.len() {
            return true;
        }
        if bigger {
            if x <= num.len() - idx {
                ans.extend(std::iter::repeat(b'1').take(num.len() - idx - x));
                ans.extend(helper.gen_min_str());
                return true;
            }
            return false;
        }
        let start = num[idx].max(b'1');
        for c in start..=b'9' {
            let digits = helper.pop(c);
            ans.push(c);
            if Solution::dfs(num, ans, idx + 1, bigger || c > num[idx], helper) {
                return true;
            } else {
                helper.push(digits);
                ans.pop();
            }
        }
        false
    }
}
