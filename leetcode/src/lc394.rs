#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "sd2[f2[e]g]i".to_owned();
        let ans = "sdfeegfeegi".to_owned();
        assert_eq!(Solution::decode_string(s), ans);
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

#[derive(Debug)]
enum Op {
    Num(usize),
    Str(String),
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut op = vec![];
        let s = s.as_bytes();
        let mut i = 0;
        while i < s.len() {
            if s[i].is_ascii_digit() {
                let mut j = i + 1;
                while s[j].is_ascii_digit() {
                    j += 1;
                }
                op.push(Op::Num(
                    std::str::from_utf8(&s[i..j])
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                ));
                i = j;
            } else if s[i] == b'[' {
                i += 1;
            } else if s[i] == b']' {
                loop {
                    let s = op.pop().unwrap();
                    let n = op.pop().unwrap();
                    match s {
                        Op::Str(s) => match n {
                            Op::Num(n) => {
                                let mut s = s.repeat(n);
                                while let Some(v) = op.pop() {
                                    if let Op::Str(t) = v {
                                        s = format!("{}{}", t, s);
                                    } else {
                                        op.push(v);
                                        break;
                                    }
                                }
                                op.push(Op::Str(s));
                                break;
                            }
                            Op::Str(t) => {
                                op.push(Op::Str(format!("{}{}", t, s)));
                            }
                        },
                        _ => unreachable!(),
                    }
                }
                i += 1;
            } else {
                let mut j = i + 1;
                while j < s.len() && s[j].is_ascii_alphabetic() {
                    j += 1;
                }
                op.push(Op::Str(std::str::from_utf8(&s[i..j]).unwrap().to_owned()));
                i = j;
            }
        }
        op.into_iter()
            .map(|o| match o {
                Op::Str(s) => s,
                _ => unreachable!(),
            })
            .collect()
    }
}
