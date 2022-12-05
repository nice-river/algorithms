use data_structures::rbtree;
use rand::seq::SliceRandom;

fn main() {
    let mut tree = rbtree::RedBlackTree::new();
    // let mut tree = BTreeMap::new();
    let n = 10000000;
    let mut arr = (1..=n).collect::<Vec<i32>>();
    arr.shuffle(&mut rand::thread_rng());
    for &v in &arr {
        tree.insert(v, v);
    }
    dbg!(tree.cmp_times);
    dbg!(tree.cmp_times as f64 / n as f64);
    dbg!((n as f64).log2());
}
