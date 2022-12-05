// mod dancing_links;
// mod skip_list;
pub mod rbtree;

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    #[test]
    fn test_insert() {
        let mut tree = crate::rbtree::RedBlackTree::new();
        // let mut tree = BTreeMap::new();
        let n = 4000000;
        let mut arr = (1..=n).collect::<Vec<i32>>();
        arr.shuffle(&mut rand::thread_rng());
        for &v in &arr {
            tree.insert(v, v);
        }
        let res = tree.debug();
        arr.sort();
        dbg!(n);
    }
}
