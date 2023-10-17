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
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let (mut c, mut d, s, t) = if a >= b {
            (a, b, b'a', b'b')
        } else {
            (b, a, b'b', b'a')
        };
        let mut ans = vec![];
        while c > d && d > 0 {
            ans.push(s);
            ans.push(s);
            ans.push(t);
            c -= 2;
            d -= 1;
        }
        while d > 0 {
            ans.push(s);
            ans.push(t);
            c -= 1;
            d -= 1;
        }
        while c > 0 {
            ans.push(s);
            c -= 1;
        }
        String::from_utf8(ans).unwrap()
    }
}
