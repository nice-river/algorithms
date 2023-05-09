#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let fruits = vec![[2, 8], [6, 3], [8, 6]];
        let fruits = fruits.into_iter().map(|v| v.to_vec()).collect();
        let start_pos = 5;
        let k = 4;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), 9);
    }
}

struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        if fruits.is_empty() {
            return 0;
        }
        let mut pos = vec![0; fruits.last().unwrap()[0] as usize + 2];
        for slice in fruits.windows(2) {
            let t = pos[slice[0][0] as usize];
            for k in slice[0][0] + 1..=slice[1][0] {
                pos[k as usize] = t + slice[0][1];
            }
        }
        *pos.last_mut().unwrap() = fruits.last().unwrap()[1] + pos[pos.len() - 2];
        let mut ans = 0;
        for i in 0..pos.len() - 1 {
            if i as i32 <= start_pos {
                if start_pos - k <= i as i32 {
                    let d = start_pos - i as i32;
                    let a = i as i32;
                    let b = (a + k - d).max(start_pos).min(pos.len() as i32 - 2);
                    ans = ans.max(pos[b as usize + 1] - pos[a as usize]);
                }
            } else {
                if start_pos + k >= i as i32 {
                    let d = i as i32 - start_pos;
                    let b = i as i32;
                    let a = (b - (k - d)).min(start_pos).max(0);
                    ans = ans.max(pos[b as usize + 1] - pos[a as usize]);
                }
            }
        }
        ans
    }
}
