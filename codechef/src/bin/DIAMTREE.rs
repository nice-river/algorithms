#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{
    fmt::Debug,
    io::{BufReader, Read},
    str::FromStr,
};

struct Reader<R> {
    reader: BufReader<R>,
    buf: Vec<u8>,
    pos: usize,
    len: usize,
}

impl<R: Read> Reader<R> {
    fn new(inner: R) -> Self {
        Self {
            reader: BufReader::new(inner),
            buf: vec![0; 4],
            pos: 0,
            len: 0,
        }
    }

    #[inline]
    fn skip_whitespaces(&mut self) {
        loop {
            while self.pos < self.len && self.buf[self.pos].is_ascii_whitespace() {
                self.pos += 1
            }
            if self.pos == self.len {
                if let Ok(n) = self.reader.read(&mut self.buf) {
                    self.len = n;
                    self.pos = 0;
                    if n != 0 {
                        continue;
                    }
                }
            }
            break;
        }
    }

    fn read<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut val = Vec::with_capacity(32);

        self.skip_whitespaces();

        loop {
            if self.pos == self.len {
                if let Ok(n) = self.reader.read(&mut self.buf) {
                    self.len = n;
                    self.pos = 0;
                    if n == 0 {
                        break;
                    }
                }
            } else {
                if self.buf[self.pos].is_ascii_whitespace() {
                    break;
                }
                val.push(self.buf[self.pos]);
                self.pos += 1;
            }
        }
        std::str::from_utf8(&val).unwrap().parse().unwrap()
    }
}

static DIRS4: [i32; 5] = [-1, 0, 1, 0, -1];
static DIRS8: [i32; 9] = [-1, -1, 0, -1, 1, 0, 1, 1, -1];

use std::collections::BinaryHeap;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let mut vis = vec![false; n + 1];
        let mut graph = vec![vec![]; n + 1];
        for _ in 0..n - 1 {
            let u: usize = reader.read();
            let v: usize = reader.read();
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut ans = 0;
        dfs(&graph, 1, &mut vis, &mut ans);
        println!("{}", ans);
    }

    Ok(())
}

fn dfs(graph: &Vec<Vec<usize>>, node: usize, vis: &mut Vec<bool>, ans: &mut i32) -> i32 {
    vis[node] = true;
    let mut heap = BinaryHeap::new();
    for &nxt_node in &graph[node] {
        if !vis[nxt_node] {
            heap.push(dfs(graph, nxt_node, vis, ans));
            if heap.len() > 2 {
                heap.pop();
            }
        }
    }
    let d = heap.iter().sum::<i32>();
    *ans = std::cmp::max(d, *ans);
    heap.into_iter().max().unwrap_or(0) + 1
}
