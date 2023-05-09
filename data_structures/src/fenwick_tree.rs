use std::ops::Add;

struct BinaryIndexedTree<T> {
    arr: Vec<T>,
}

impl<T: Add<Output = T> + Default + Copy> BinaryIndexedTree<T> {
    fn new(sz: usize) -> Self {
        Self {
            arr: vec![T::default(); sz],
        }
    }

    fn update(&mut self, mut idx: usize, val: T) {
        while idx < self.arr.len() {
            self.arr[idx] = self.arr[idx] + val;
            idx += (!idx + 1) & idx;
        }
    }

    fn get(&self, mut idx: usize) -> T {
        assert!(idx < self.arr.len());
        let mut res = T::default();
        while idx > 0 {
            res = res + self.arr[idx];
            idx -= (!idx + 1) & idx;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::Rng;

    #[test]
    fn test_bit() {
        let n = 5000;
        let mut bit = BinaryIndexedTree::<i32>::new(n + 1);
        let mut arr = vec![0; n];
        let mut random = rand::thread_rng();

        let t = 100000;
        for _ in 0..t {
            let idx = random.gen::<usize>() % n;
            let val = random.gen::<i32>() % 10;
            bit.update(idx + 1, val);
            arr[idx] += val;
        }
        for i in 0..n {
            assert_eq!(bit.get(i + 1), arr[0..=i].iter().sum::<i32>());
        }
    }
}
