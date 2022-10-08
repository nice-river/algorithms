use std::collections::HashMap;
use std::collections::HashSet;

struct ThroneInheritance {
	king: String,
	deaths: HashSet<String>,
    children: HashMap<String, Vec<String>>,
}

impl ThroneInheritance {
    fn new(king_name: String) -> Self {
	    let mut map = HashMap::new();
	    map.insert(king_name.clone(), vec![]);
	    Self {
		    king: king_name,
            deaths: HashSet::new(),
		    children: map,
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
	    self.children.entry(parent_name).or_insert(vec![]).push(child_name.clone());
	    self.children.insert(child_name, vec![]);
    }

    fn death(&mut self, name: String) {
	    self.deaths.insert(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
	    let mut ret = vec![];
	    self.dfs(&self.king, &mut ret);
	    ret
    }

	fn dfs(&self, name: &String, ans: &mut Vec<String>) {
		if !self.deaths.contains(name) {
			ans.push(name.clone());
		}
		if let Some(children) = self.children.get(name) {
			for child in children {
				self.dfs(child, ans);
			}
		}
	}
}