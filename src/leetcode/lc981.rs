use std::collections::{HashMap, BTreeMap};

struct TimeMap {
	data: HashMap<String, BTreeMap<i32, String>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
	    Self {
		    data: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
	    let e = self.data.entry(key).or_insert(BTreeMap::new());
	    e.insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
	    if let Some(x) = self.data.get(&key) {
		    let r = x.range(..=timestamp);
		    if let Some((_, t)) = r.last() {
			    t.clone()
		    } else {
			    String::new()
		    }
	    } else {
		    String::new()
	    }
    }
}
