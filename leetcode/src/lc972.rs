#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "0.(52)";
        let t = "0.5(25)";
        assert!(Solution::is_rational_equal(s.to_owned(), t.to_owned()));
    }

    #[test]
    fn test1() {
        let s = "0.9(9999)";
        let t = "1.0000";
        assert!(Solution::is_rational_equal(s.to_owned(), t.to_owned()));
    }

    #[test]
    fn test2() {
        let s = "1.00";
        let t = "1.0000";
        assert!(Solution::is_rational_equal(s.to_owned(), t.to_owned()));
    }

    #[test]
    fn test3() {
        let s = "0.1666(6)";
        let t = "0.166(66)";
        assert!(Solution::is_rational_equal(s.to_owned(), t.to_owned()));
    }

    #[test]
    fn test4() {
        let s = "0.00(0)";
        let t = "0.";
        assert!(Solution::is_rational_equal(s.to_owned(), t.to_owned()));
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

struct Solution {}

struct Rational {
    a: i32,
    b: Vec<i32>,
    c: Vec<i32>,
}

impl Rational {
    fn new(s: String) -> Self {
        let mut s = s.split('.');
        let mut a = s.next().unwrap().parse::<i32>().unwrap();
        let mut b = Vec::new();
        let mut c = Vec::new();
        if let Some(s) = s.next() {
            let s = s.as_bytes();
            for i in 0..s.len() {
                if s[i] == b'(' {
                    b = std::str::from_utf8(&s[..i])
                        .unwrap()
                        .as_bytes()
                        .iter()
                        .map(|&ch| (ch - b'0') as i32)
                        .collect();
                    c = std::str::from_utf8(&s[i + 1..s.len() - 1])
                        .unwrap()
                        .as_bytes()
                        .iter()
                        .map(|&ch| (ch - b'0') as i32)
                        .collect();
                }
            }
            if c.is_empty() {
                b = std::str::from_utf8(&s[..])
                    .unwrap()
                    .as_bytes()
                    .iter()
                    .map(|&ch| (ch - b'0') as i32)
                    .collect();
            }
        }
        if !c.is_empty() {
            while !b.is_empty() {
                if b[b.len() - 1] == c[c.len() - 1] {
                    let d = b.pop().unwrap();
                    c.insert(0, d);
                    c.pop();
                } else {
                    break;
                }
            }
            for i in 1..c.len() {
                if c.len() % i == 0 {
                    let mut found = true;
                    for k in (i..c.len()).step_by(i) {
                        if c[k..k + i] != c[..i] {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        c.truncate(i);
                        break;
                    }
                }
            }
            if c.len() == 1 {
                if c[0] == 9 {
                    if b.is_empty() {
                        a += 1;
                    } else {
                        let mut d = 1;
                        for i in (0..b.len()).rev() {
                            b[i] += d;
                            if b[i] >= 10 {
                                b[i] = 0;
                            } else {
                                d = 0;
                                break;
                            }
                        }
                        if d > 0 {
                            a += 1;
                        }
                    }
                    c.clear();
                } else if c[0] == 0 {
                    c.clear();
                }
            }
        }
        if c.is_empty() {
            if b.iter().all(|d| *d == 0) {
                b.clear();
            }
        }
        Self { a, b, c }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

impl Eq for Rational {}

impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        Rational::new(s) == Rational::new(t)
    }
}
