struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let times = vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]];
        let n = 4;
        let k = 2;
        let ans = 2;
        assert_eq!(Solution::network_delay_time(times, n, k), ans);
	}

    #[test]
    fn test1() {
        let times = vec![vec![1, 2, 3],vec![2, 3, 1],vec![4, 2, 1], vec![1, 4, 1]];
        let n = 4;
        let k = 1;
        let ans = 3;
        assert_eq!(Solution::network_delay_time(times, n, k), ans);
    }
}

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut gph = HashMap::new();
        for time in times {
            gph.entry(time[0] as usize).or_insert(Vec::new()).push((time[1] as usize, time[2]));
        }
        let n = n as usize;
        let k = k as usize;
        let mut dist = vec![i32::MAX; n + 1];
        dist[k] = 0;

        let mut que = VecDeque::new();
        que.push_back(k);

        while !que.is_empty() {
            let node = que.pop_front().unwrap();
            if let Some(nxts) = gph.get(&node) {
                for nxt in nxts {
                    if dist[node] + nxt.1 < dist[nxt.0] {
                        dist[nxt.0] = dist[node] + nxt.1;
                        que.push_back(nxt.0);
                    }
                }
            }
        }

        let ans = dist.into_iter().skip(1).max().unwrap();
        if ans == i32::MAX { -1 } else { ans }
    }
}