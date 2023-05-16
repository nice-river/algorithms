#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

use std::collections::HashMap;

struct FrequencyTracker {
    kv: HashMap<i32, i32>,
    vk: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        Self {
            kv: HashMap::new(),
            vk: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let mut old = 0;
        let mut new = 0;
        if let Some(v) = self.kv.get_mut(&number) {
            old = *v;
            *v += 1;
            new = old + 1;
        } else {
            old = 0;
            self.kv.insert(number, 1);
            new = old + 1;
        }
        if old != 0 {
            *self.vk.entry(old).or_insert(0) -= 1;
        }
        *self.vk.entry(new).or_insert(0) += 1;
    }

    fn delete_one(&mut self, number: i32) {
        let mut old = 0;
        let mut new = 0;
        if let Some(v) = self.kv.get_mut(&number) {
            old = *v;
            *v -= 1;
            new = old - 1;
        } else {
            return;
        }
        if new != 0 {
            *self.vk.entry(old).or_insert(0) -= 1;
            *self.vk.entry(new).or_insert(0) += 1;
        } else {
            *self.vk.entry(old).or_insert(0) -= 1;
            self.kv.remove(&number);
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        if frequency == 0 {
            return true;
        }
        if let Some(t) = self.vk.get(&frequency) {
            if *t != 0 {
                return true;
            }
        }
        false
    }
}
