use std::collections::HashMap;

struct MapSum {
    map: HashMap<String, i32>,
    prefix_sum: HashMap<String, i32>,
}

impl MapSum {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            prefix_sum: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let val = val - self.map.insert(key.clone(), val).unwrap_or(0);
        let key = key.as_bytes();
        for i in 1..=key.len() {
            let k = String::from_utf8_lossy(&key[0..i]).to_string();
            *self.prefix_sum.entry(k).or_insert(0) += val;
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        *self.prefix_sum.get(&prefix).unwrap_or(&0)
    }
}
