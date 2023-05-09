#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let courses = [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]];
        let ans = 3;
        let mut courses = courses.to_vec();
        let courses = courses.into_iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::schedule_course(courses), ans);
    }

    #[test]
    fn test1() {
        let courses = [[1, 2]];
        let ans = 1;
        let mut courses = courses.to_vec();
        let courses = courses.into_iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::schedule_course(courses), ans);
    }

    #[test]
    fn test2() {
        let courses = [[1, 2], [2, 4]];
        let ans = 2;
        let mut courses = courses.to_vec();
        let courses = courses.into_iter().map(|v| v.to_vec()).collect();
        assert_eq!(Solution::schedule_course(courses), ans);
    }
}

struct Solution {}

use std::collections::BinaryHeap;

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_unstable_by_key(|v| v[1]);
        let mut heap = BinaryHeap::new();
        let mut t = 0;
        for course in courses {
            t += course[0];
            heap.push(course[0]);
            if t > course[1] {
                t -= heap.pop().unwrap();
            }
        }
        heap.len() as i32
    }
}
