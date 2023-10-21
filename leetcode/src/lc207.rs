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
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut gph = vec![vec![]; num_courses as usize];
        let mut in_degrees = vec![0; num_courses as usize];
        for req in prerequisites {
            gph[req[1] as usize].push(req[0] as usize);
            in_degrees[req[0] as usize] += 1;
        }
        let mut avail = vec![];
        let mut done = 0;
        for i in 0..n {
            if in_degrees[i] == 0 {
                avail.push(i);
            }
        }
        while let Some(i) = avail.pop() {
            done += 1;
            for &nxt in &gph[i] {
                in_degrees[nxt] -= 1;
                if in_degrees[nxt] == 0 {
                    avail.push(nxt);
                }
            }
        }
        done == n
    }
}
