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

use crate::*;

struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut arr = vec![nums[0]];
        for num in nums.into_iter().skip(1) {
            if &num > arr.last().unwrap() {
                arr.push(num);
            } else {
                let mut l = 0;
                let mut r = arr.len();
                while l < r {
                    let mid = (l + r) / 2;
                    if arr[mid] < num {
                        l = mid + 1;
                    } else if arr[mid] == num {
                        r = mid;
                        break;
                    } else {
                        r = mid;
                    }
                }
                arr[r] = num;
            }
        }

        arr.len() as i32
    }
}
