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
    pub fn remove_trailing_zeros(mut num: String) -> String {
        let mut k = None;
        while let Some(x) = num.pop() {
            if x != '0' {
                k = Some(x);
                break;
            }
        }
        if let Some(x) = k {
            num.push(x);
        }
        num
    }
}
