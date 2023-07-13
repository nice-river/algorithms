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
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut arr = (1..=n).collect::<Vec<_>>();
        let mut p = 0;
        while arr.len() > 1 {
            let t = (p + k - 1) % arr.len();
            arr.remove(t);
            p = t % arr.len();
        }
        arr[0] as i32
    }
}
