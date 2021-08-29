#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sol = Solution::new(vec![1, 2, 3, 4]);
        let mut s = vec![0, 0, 0, 0];
        for _ in 0..1000000 {
            s[sol.pick_index() as usize] += 1;
        }
        println!("{:?}", s);
    }
}

use rand;

struct Solution {
    presum: Vec<i32>,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut presum = vec![0; w.len() + 1];
        for (i, n) in w.into_iter().enumerate() {
            presum[i + 1] = presum[i] + n;
        }
        Self { presum }
    }

    fn pick_index(&self) -> i32 {
        let largest = *self.presum.last().unwrap();
        let gen = (rand::random::<u32>() % largest as u32 + 1) as i32;
        let idx = lower_bound(&self.presum, 0, self.presum.len(), &gen);
        idx as i32 - 1
    }
}

fn lower_bound<T>(arr: &[T], start: usize, end: usize, target: &T) -> usize
where
    T: Ord,
{
    let (mut start, mut end) = (start, end);
    while start < end {
        let mid = start + (end - start) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => {
                start = mid + 1;
            }
            _ => {
                end = mid;
            }
        }
    }
    start
}
