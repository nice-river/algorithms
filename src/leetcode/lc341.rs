#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct NestedIterator {
  list: Vec<i32>,
  cur_idx: usize,
}

impl NestedIterator {
  fn new(nested_list: Vec<NestedInteger>) -> Self {
    let mut list = Vec::<i32>::new();

    fn build_list(nested_list: &Vec<NestedInteger>, list: &mut Vec<i32>) {
      for elem in nested_list {
        match elem {
          NestedInteger::Int(x) => {
            list.push(*x);
          }
          NestedInteger::List(arr) => {
            build_list(arr, list);
          }
        }
      }
    }

    build_list(&nested_list, &mut list);

    Self {
      list,
      cur_idx: 0,
    }

  }

  fn next(&mut self) -> i32 {
    assert!(self.has_next());
    let ret = self.list[self.cur_idx];
    self.cur_idx += 1;
    ret
  }

  fn has_next(&self) -> bool {
    self.cur_idx != self.list.len()
  }
}
