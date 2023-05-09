struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


use std::collections::{HashMap, HashSet};


impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
		let mut foods = HashSet::new();
		let mut table_orders = HashMap::new();
        for order in orders {
            let table_num = order[1].parse::<i32>().unwrap();
			let food = order[2].clone();
			let item = table_orders.entry(table_num).or_insert(HashMap::new());
            *item.entry(food.clone()).or_insert(0) += 1;
			foods.insert(food);
		}
        let mut foods = foods.into_iter().collect::<Vec<_>>();
		foods.sort_unstable();
		let mut ans = vec![vec!["Table".to_string()]];
		ans.last_mut().unwrap().extend(foods.iter().cloned());
		let mut table_orders = table_orders.into_iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
		table_orders.sort_unstable_by_key(|(k, _)| *k);

		for (table_num, table_info) in table_orders {
            let mut row = Vec::with_capacity(1 + foods.len());
			row.push(table_num.to_string());
			for food in foods.iter() {
                row.push(table_info.get(food).unwrap_or(&0).to_string());
			}
            ans.push(row);
		}
        ans
    }
}