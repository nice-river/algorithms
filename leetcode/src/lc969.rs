#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let arr = vec![3, 2, 4, 1];
        dbg!(Solution::pancake_sort(arr));
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
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut ordered = arr.clone();
        ordered.sort_by_key(|x| -x);
        let mut ans = vec![];
        for i in 0..ordered.len() - 1 {
            for j in 0..arr.len() {
                if arr[j] == ordered[i] {
                    ans.push(j as i32 + 1);
                    ans.push((arr.len() - i) as i32);
                    arr[0..j + 1].reverse();
                    arr[0..ordered.len() - i].reverse();
                    break;
                }
            }
        }
        ans
    }
}
