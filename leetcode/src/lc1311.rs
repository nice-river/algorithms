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

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        level: i32,
    ) -> Vec<String> {
        let mut levels = vec![false; watched_videos.len()];
        let mut que = VecDeque::new();
        que.push_back((id as usize, 0));
        levels[id as usize] = true;
        while let Some((id, l)) = que.pop_front() {
            if l == level {
                que.push_back((id, l));
                break;
            }
            for &friend in &friends[id] {
                if !levels[friend as usize] {
                    levels[friend as usize] = true;
                    que.push_back((friend as usize, l + 1));
                }
            }
        }

        let mut map = HashMap::new();
        while let Some((id, _)) = que.pop_front() {
            for video in &watched_videos[id] {
                *map.entry(video.clone()).or_insert(0) += 1;
            }
        }
        let mut ans = map.into_iter().collect::<Vec<_>>();
        ans.sort_by_key(|e| (e.1, e.0.clone()));
        ans.into_iter().map(|e| e.0).collect()
    }
}
