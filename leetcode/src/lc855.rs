#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        // 输入：["ExamRoom","seat","seat","seat","seat","leave","seat"], [[10],[],[],[],[],[4],[]]
        // 输出：[null,0,9,4,2,null,5]
        let mut room = ExamRoom::new(10);
        assert_eq!(room.seat(), 0);
        assert_eq!(room.seat(), 9);
        assert_eq!(room.seat(), 4);
        assert_eq!(room.seat(), 2);
        room.leave(4);
        assert_eq!(room.seat(), 5);
        assert_eq!(room.seat(), 7);
        room.leave(5);
        room.leave(0);
        room.leave(9);
        assert_eq!(room.seat(), 0);
        assert_eq!(room.seat(), 4);
        assert_eq!(room.seat(), 9);
    }

    #[test]
    fn test1() {
        let ops = [
            "ExamRoom", "seat", "leave", "seat", "leave", "seat", "leave", "seat", "leave", "seat",
            "leave",
        ];
        let args = vec![
            vec![1000000000],
            vec![],
            vec![0],
            vec![],
            vec![0],
            vec![],
            vec![0],
            vec![],
            vec![0],
            vec![],
            vec![0],
        ];
        let mut room = None;
        for (&op, arg) in ops.iter().zip(args.iter()) {
            if op == "ExamRoom" {
                room = Some(ExamRoom::new(arg[0]));
            } else if op == "seat" {
                room.as_mut().unwrap().seat();
            } else {
                room.as_mut().unwrap().leave(arg[0]);
            }
        }
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

use std::collections::BTreeSet;
use std::collections::HashMap;

struct ExamRoom {
    n: i32,
    ivls: BTreeSet<(i32, i32, i32)>,
    p2ivls: HashMap<i32, Vec<i32>>,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        let mut s = Self {
            n,
            ivls: BTreeSet::new(),
            p2ivls: HashMap::new(),
        };
        s.ivls.insert((-n, -1, n));
        s
    }

    fn seat(&mut self) -> i32 {
        let ivl = self.ivls.iter().next().unwrap();
        let d = ivl.0;
        let a = ivl.1;
        let b = ivl.2;
        let n = self.n;
        let p = if a == -1 {
            if b == n {
                self.ivls.insert((-(n - 1), 0, n));
            } else {
                self.ivls.insert((-b / 2, 0, b));
                self.p2ivls.get_mut(&b).unwrap()[0] = 0;
            }
            self.p2ivls.insert(0, vec![-1, b]);
            0
        } else if b == n {
            self.ivls.insert((-(n - 1 - a) / 2, a, n - 1));
            self.p2ivls.get_mut(&a).unwrap()[1] = n - 1;
            self.p2ivls.insert(n - 1, vec![a, b]);
            n - 1
        } else {
            let p = (a + b) / 2;
            self.ivls.insert((-(p - a) / 2, a, p));
            self.ivls.insert((-(b - p) / 2, p, b));
            self.p2ivls.get_mut(&a).unwrap()[1] = p;
            self.p2ivls.get_mut(&b).unwrap()[0] = p;
            self.p2ivls.insert(p, vec![a, b]);
            p
        };
        self.ivls.remove(&(d, a, b));
        p
    }

    fn leave(&mut self, p: i32) {
        let n = self.n;
        let a = self.p2ivls.get(&p).unwrap()[0];
        let b = self.p2ivls.get(&p).unwrap()[1];
        if a == -1 {
            self.ivls.remove(&(-p, -1, p));
            if b == n {
                self.ivls.remove(&(-(n - 1 - p), p, n));
                self.ivls.insert((-n, -1, n));
            } else {
                self.ivls.remove(&(-(b - p) / 2, p, b));
                self.ivls.insert((-b, -1, b));
                self.p2ivls.get_mut(&b).unwrap()[0] = -1;
            }
        } else if b == n {
            self.ivls.remove(&(-(p - a) / 2, a, p));
            self.ivls.remove(&(-(n - 1 - p), p, n));
            self.ivls.insert((-(n - 1 - a), a, n));
            self.p2ivls.get_mut(&a).unwrap()[1] = n;
        } else {
            self.ivls.remove(&(-(p - a) / 2, a, p));
            self.ivls.remove(&(-(b - p) / 2, p, b));
            self.ivls.insert((-(b - a) / 2, a, b));
            self.p2ivls.get_mut(&a).unwrap()[1] = b;
            self.p2ivls.get_mut(&b).unwrap()[0] = a;
        }
        self.p2ivls.remove(&p);
    }
}
