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

use std::collections::HashMap;

struct Cashier {
    n: i32,
    idx: i32,
    discount: f64,
    prices: HashMap<i32, i32>,
}

impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for (product, price) in products.into_iter().zip(prices.into_iter()) {
            map.insert(product, price);
        }
        Self {
            n,
            idx: 1,
            discount: discount as f64,
            prices: map,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut cost = 0.0;
        for (p, a) in product.into_iter().zip(amount.into_iter()) {
            cost += (*self.prices.get(&p).unwrap_or(&0) * a) as f64;
        }
        if self.idx == self.n {
            cost = cost - (cost * self.discount) / 100.0;
            self.idx = 0;
        }
        self.idx += 1;
        cost
    }
}
