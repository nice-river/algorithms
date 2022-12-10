// mod dancing_links;
// mod skip_list;
pub mod rbtree;

#[cfg(test)]
mod tests {
    mod rbtree {
        use crate::rbtree::RedBlackTree;
        use rand::{seq::SliceRandom, Rng};
        use std::collections::BTreeMap;

        #[test]
        fn test_insert() {
            let mut tree = crate::rbtree::RedBlackTree::new();
            let n = 100000;
            let mut arr = (1..=n).collect::<Vec<usize>>();
            arr.shuffle(&mut rand::thread_rng());
            for &v in &arr {
                tree.insert(v, v);
            }
            arr.shuffle(&mut rand::thread_rng());
            for &v in &arr {
                assert_eq!(tree.insert(v, v), Some(v));
            }
            assert_eq!(tree.len(), n);
            arr.shuffle(&mut rand::thread_rng());
            for &kth in &arr {
                assert_eq!(tree.kth_smallest_key(kth), Some(&kth));
            }
        }

        #[test]
        fn test_remove() {
            let mut tree = crate::rbtree::RedBlackTree::new();
            let n = 100000;
            let mut arr = (1..=n).collect::<Vec<usize>>();
            arr.shuffle(&mut rand::thread_rng());
            for &v in &arr {
                tree.insert(v, v);
            }
            assert_eq!(tree.len(), n);
            for i in (1..=n).step_by(2) {
                assert_eq!(tree.remove(&i), Some(i));
                assert_eq!(tree.remove(&i), None);
            }
            for i in 1..=n / 2 {
                assert_eq!(tree.kth_smallest_key(i), Some(&(i * 2)));
            }
            assert_eq!(tree.len(), n / 2);
        }

        #[test]
        fn test_insert_remove_get_kth() {
            let n = 100000;
            let test_case = 10;
            for _ in 0..test_case {
                let mut stdtree = BTreeMap::new();
                let mut rbtree = RedBlackTree::new();
                for _ in 0..n * 5 {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen::<i32>() % n - n / 2;
                    let y = rng.gen::<i32>() % n;
                    let op = rng.gen::<i32>() % 3;
                    if op == 0 {
                        assert_eq!(stdtree.insert(x, y), rbtree.insert(x, y));
                    } else if op == 1 {
                        assert_eq!(stdtree.remove(&x), rbtree.remove(&x));
                    } else {
                        assert_eq!(stdtree.get(&x), rbtree.get(&x));
                    }
                }
                assert_eq!(stdtree.len(), rbtree.len());
                let x: Vec<(i32, i32)> = stdtree.into_iter().collect();
                for i in 0..x.len() {
                    let kth = rbtree.kth_smallest_key(i + 1);
                    assert!(kth.is_some());
                    let &kth = kth.unwrap();
                    assert_eq!((kth, *rbtree.get(&kth).unwrap()), x[i]);
                }
            }
        }

        #[test]
        fn test_iter() {
            let n = 100000;
            let test_case = 10;
            for _ in 0..test_case {
                let mut stdtree = BTreeMap::new();
                let mut rbtree = RedBlackTree::new();
                for _ in 0..n * 5 {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen::<i32>() % n - n / 2;
                    let y = rng.gen::<i32>() % n;
                    let op = rng.gen::<i32>() % 2;
                    if op == 0 {
                        assert_eq!(stdtree.insert(x, y), rbtree.insert(x, y));
                    } else if op == 1 {
                        assert_eq!(stdtree.remove(&x), rbtree.remove(&x));
                    }
                }
                assert_eq!(stdtree.len(), rbtree.len());
                let x = stdtree.iter().collect::<Vec<_>>();
                let y = rbtree.iter().collect::<Vec<_>>();
                let z = rbtree.iter().collect::<Vec<_>>(); // multi iter
                assert_eq!(x, y);
                let x = stdtree.iter().rev().collect::<Vec<_>>();
                let y = rbtree.iter().rev().collect::<Vec<_>>();
                assert_eq!(x, y);
                assert_eq!(z, stdtree.iter().collect::<Vec<_>>());
            }
        }

        #[test]
        fn test_into_iter() {
            let n = 100000;
            let test_case = 10;
            for _ in 0..test_case {
                let mut stdtree = BTreeMap::new();
                let mut rbtree = RedBlackTree::new();
                for _ in 0..n * 5 {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen::<i32>() % n - n / 2;
                    let y = rng.gen::<i32>() % n;
                    let op = rng.gen::<i32>() % 2;
                    if op == 0 {
                        assert_eq!(stdtree.insert(x, y), rbtree.insert(x, y));
                    } else if op == 1 {
                        assert_eq!(stdtree.remove(&x), rbtree.remove(&x));
                    }
                }
                assert_eq!(stdtree.len(), rbtree.len());
                let x = stdtree.iter().collect::<Vec<_>>();
                let y = rbtree.iter().collect::<Vec<_>>();
                assert_eq!(x, y);
                drop(x);
                drop(y);
                let x = stdtree.into_iter().rev().take(n as usize / 2).collect::<Vec<_>>();
                let y = rbtree.into_iter().rev().take(n as usize / 2).collect::<Vec<_>>();
                assert_eq!(x, y);
            }
        }

        #[test]
        fn test_iter_mut() {
            let n = 100000;
            let test_case = 10;
            for _ in 0..test_case {
                let mut stdtree = BTreeMap::new();
                let mut rbtree = RedBlackTree::new();
                for _ in 0..n * 5 {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen::<i32>() % n - n / 2;
                    let y = rng.gen::<i32>() % n;
                    let op = rng.gen::<i32>() % 2;
                    if op == 0 {
                        assert_eq!(stdtree.insert(x, y), rbtree.insert(x, y));
                    } else if op == 1 {
                        assert_eq!(stdtree.remove(&x), rbtree.remove(&x));
                    }
                }
                assert_eq!(stdtree.len(), rbtree.len());
                stdtree.iter_mut().for_each(|(k, v)| *v = *v * 2 + *k);
                rbtree.iter_mut().for_each(|(k, v)| *v = *v * 2 + *k);
                let x = stdtree.iter_mut().rev().collect::<Vec<_>>();
                let y = rbtree.iter_mut().rev().collect::<Vec<_>>();
                // let z = rbtree.iter_mut().rev().collect::<Vec<_>>(); // this line should fail compile
                assert_eq!(x, y);
            }
        }
    }
}
