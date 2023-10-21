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

use std::collections::{btree_map::Entry, BTreeMap};

struct MinStack {
    stk: Vec<i32>,
    mini: BTreeMap<i32, i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stk: vec![],
            mini: BTreeMap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stk.push(val);
        *self.mini.entry(val).or_insert(0) += 1;
    }

    fn pop(&mut self) {
        if let Some(e) = self.stk.pop() {
            if let Entry::Occupied(mut entry) = self.mini.entry(e) {
                *entry.get_mut() -= 1;
                if entry.get() == &0 {
                    entry.remove_entry();
                }
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stk.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mini.iter().next().unwrap().0
    }
}
