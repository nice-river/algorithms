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

use std::collections::VecDeque;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let root: usize = reader.read();
        let k: i64 = reader.read();
        let mut tl = vec![0; n + 1];
        for i in 1..=n {
            tl[i] = reader.read();
        }
        let mut gph = vec![vec![]; n + 1];
        for _ in 1..n {
            let u: usize = reader.read();
            let v: usize = reader.read();
            gph[u].push(v);
            gph[v].push(u);
        }
        let mut res = tl.clone();
        let mut que = VecDeque::with_capacity(n);
        dfs(&gph, 0, root, &mut que, &tl, &mut res, &mut 0, k);
        println!(
            "{}",
            tl.iter().zip(res.iter()).map(|(a, b)| a - b).sum::<i64>() * 2
        );
    }

    Ok(())
}

fn dfs(
    gph: &Vec<Vec<usize>>,
    parent: usize,
    node: usize,
    que: &mut VecDeque<usize>,
    tl: &Vec<i64>,
    res: &mut Vec<i64>,
    val: &mut i64,
    k: i64,
) {
    *val = *val + tl[node];
    que.push_back(node);
    while *val > k {
        let i = que.pop_front().unwrap();
        if res[i] > (*val - k) {
            que.push_front(i);
            res[i] = res[i] - (*val - k);
            *val = k;
            break;
        } else {
            *val -= res[i];
            res[i] = 0;
        }
    }
    for &nxt in &gph[node] {
        if parent != nxt {
            dfs(gph, node, nxt, que, tl, res, val, k);
        }
    }
    que.pop_back();
    *val = (*val - tl[node]).max(0);
}
