use std::cmp::min;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let rectangles = [
            [1, 1, 3, 3],
            [3, 1, 4, 2],
            [3, 2, 4, 4],
            [1, 3, 2, 4],
            [2, 3, 3, 4],
        ];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert!(Solution::is_rectangle_cover(rectangles));
    }

    #[test]
    fn test1() {
        let rectangles = [[1, 1, 3, 3], [1, 3, 3, 4]];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert!(Solution::is_rectangle_cover(rectangles));
    }

    #[test]
    fn test2() {
        let rectangles = [[0, 0, 1, 1], [0, 1, 1, 2], [0, 2, 1, 3], [0, 3, 1, 4]];
        let rectangles = rectangles.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert!(Solution::is_rectangle_cover(rectangles));
    }
}

struct Solution {}

#[derive(Default, Debug, Copy, Clone)]
struct Node {
    left_bound: usize,
    right_bound: usize,
    val: i32,
    flat: bool,
}

struct SegmentTree {
    nodes: Vec<Node>,
}

impl SegmentTree {
    fn new(sz: usize, val: i32) -> Self {
        let mut nodes = vec![Node::default(); sz * 4];
        let mut tree = Self { nodes };
        tree.init(0, 0, sz, val);
        tree
    }

    fn init(&mut self, root: usize, left: usize, right: usize, val: i32) {
        let node = &mut self.nodes[root];
        node.left_bound = left;
        node.right_bound = right;
        node.val = val;
        node.flat = true;
        if left == right {
            return;
        }
        let mid = (left + right) / 2;
        let lc = (root << 1) + 1;
        let rc = (root << 1) + 2;
        self.init(lc, left, mid, val);
        self.init(rc, mid + 1, right, val);
    }

    fn push_down(&mut self, root: usize) {
        if self.nodes[root].left_bound == self.nodes[root].right_bound || !self.nodes[root].flat {
            return;
        }
        let lc = (root << 1) + 1;
        let rc = (root << 1) + 2;
        self.nodes[lc].val = self.nodes[root].val;
        self.nodes[rc].val = self.nodes[root].val;
        self.nodes[lc].flat = true;
        self.nodes[rc].flat = true;
    }

    fn insert(&mut self, root: usize, left: usize, right: usize, val: i32) {
        self.push_down(root);
        let node = &mut self.nodes[root];
        if right < node.left_bound || node.right_bound < left {
            return;
        }
        if node.left_bound >= left && node.right_bound <= right {
            node.flat = true;
            node.val = val;
            return;
        }
        let lc = (root << 1) + 1;
        let rc = (root << 1) + 2;
        self.insert(lc, left, right, val);
        self.insert(rc, left, right, val);
        if self.nodes[lc].flat && self.nodes[rc].flat && self.nodes[lc].val == self.nodes[rc].val {
            self.nodes[root].val = self.nodes[lc].val;
            self.nodes[root].flat = true;
        } else {
            self.nodes[root].flat = false;
        }
    }

    fn query(&mut self, root: usize, left: usize, right: usize) -> Result<Option<i32>, ()> {
        self.push_down(root);
        let node = &self.nodes[root];
        if right < node.left_bound || node.right_bound < left {
            return Ok(None);
        }
        if node.left_bound >= left && node.right_bound <= right {
            return if node.flat {
                Ok(Some(node.val))
            } else {
                Err(())
            };
        }
        let lc = (root << 1) + 1;
        let rc = (root << 1) + 2;

        let left_val = self.query(lc, left, right);
        let right_val = self.query(rc, left, right);
        match (left_val, right_val) {
            (_, Err(())) | (Err(()), _) => Err(()),
            (Ok(Some(v)), Ok(None)) | (Ok(None), Ok(Some(v))) => Ok(Some(v)),
            (Ok(Some(x)), Ok(Some(y))) => {
                if x == y {
                    Ok(Some(x))
                } else {
                    Err(())
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut rectangles = rectangles
            .into_iter()
            .map(|r| (r[1], r[0], r[2] - r[0], r[3] - r[1]))
            .collect::<Vec<_>>();
        rectangles.sort_unstable();
        let min_x = rectangles.iter().map(|v| v.1).min().unwrap();
        let max_x = rectangles.iter().map(|v| v.1 + v.2).max().unwrap();
        let mut tree = SegmentTree::new((max_x - min_x) as usize, rectangles[0].0);
        for rectangle in rectangles {
            let left = rectangle.1 - min_x;
            let right = left + rectangle.2 - 1;
            match tree.query(0, left as usize, right as usize) {
                Err(_) => return false,
                Ok(Some(v)) => {
                    if v != rectangle.0 {
                        return false;
                    }
                    tree.insert(0, left as usize, right as usize, v + rectangle.3);
                }
                _ => unreachable!(),
            }
        }
        let left = 0;
        let right = max_x - min_x - 1;
        match tree.query(0, left as usize, right as usize) {
            Err(_) => false,
            Ok(Some(v)) => true,
            _ => unreachable!(),
        }
    }
}
