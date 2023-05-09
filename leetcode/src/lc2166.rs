#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use std::collections::HashSet;

struct Bitset {
    flip: bool,
    size: i32,
    bits: HashSet<i32>,
}

impl Bitset {
    fn new(size: i32) -> Self {
        Self {
            flip: false,
            size,
            bits: HashSet::new(),
        }
    }

    fn fix(&mut self, idx: i32) {
        if self.flip {
            self.bits.remove(&idx);
        } else {
            self.bits.insert(idx);
        }
    }

    fn unfix(&mut self, idx: i32) {
        if self.flip {
            self.bits.insert(idx);
        } else {
            self.bits.remove(&idx);
        }
    }

    fn flip(&mut self) {
        self.flip ^= true;
    }

    fn all(&self) -> bool {
        if self.flip {
            self.bits.is_empty()
        } else {
            self.bits.len() == self.size as usize
        }
    }

    fn one(&self) -> bool {
        if self.flip {
            self.bits.len() != self.size as usize
        } else {
            !self.bits.is_empty()
        }
    }

    fn count(&self) -> i32 {
        if self.flip {
            self.size - self.bits.len() as i32
        } else {
            self.bits.len() as i32
        }
    }

    fn to_string(&self) -> String {
        if self.flip {
            let mut ans = vec!["1"; self.size as usize];
            for &bit in self.bits.iter() {
                ans[bit as usize] = "0"
            }
            ans.join("")
        } else {
            let mut ans = vec!["0"; self.size as usize];
            for &bit in self.bits.iter() {
                ans[bit as usize] = "1"
            }
            ans.join("")
        }
    }
}
