#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let arr1 = vec![1, 0, 1];
        let arr2 = vec![0, 0, 1];
        let ans = vec![1, 1, 0, 1, 0];
        assert_eq!(Solution::add_negabinary(arr1, arr2), ans);
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

impl Solution {
    pub fn add_negabinary(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut ans = vec![];
        let mut c = 0;
        arr1.reverse();
        arr2.reverse();
        while i < arr1.len() || i < arr2.len() || c != 0 {
            let a = if i >= arr1.len() { 0 } else { arr1[i] };
            let b = if i >= arr2.len() { 0 } else { arr2[i] };
            let k = a + b + c;
            match k {
                -1 => {
                    ans.push(1);
                    c = 1;
                }
                0 => {
                    ans.push(0);
                    c = 0;
                }
                1 => {
                    ans.push(1);
                    c = 0;
                }
                2 => {
                    ans.push(0);
                    c = -1;
                }
                3 => {
                    ans.push(1);
                    c = -1;
                }
                _ => unreachable!(),
            }
            i += 1;
        }
        while !ans.is_empty() && ans.last().unwrap() == &0 {
            ans.pop();
        }
        if ans.is_empty() {
            ans.push(0);
        }
        ans.reverse();
        ans
    }
}
