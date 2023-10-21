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
    pub fn greatest_letter(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let mut x = None;
        for i in 0..s.len() {
            let l = s[i].to_lowercase().next().unwrap();
            let u = s[i].to_uppercase().next().unwrap();
            if s.contains(&l) && s.contains(&u) {
                if let Some(c) = x {
                    if u > c {
                        x = Some(u);
                    }
                } else {
                    x = Some(u);
                }
            }
        }
        x.map(|c| c.to_string()).unwrap_or("".to_owned())
    }
}
