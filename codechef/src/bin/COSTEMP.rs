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

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);

    let n: usize = reader.read();
    let mut gph = vec![vec![]; n];
    let mut sub_dist = vec![vec![]; n];
    for _ in 1..n {
        let u: usize = reader.read();
        let v: usize = reader.read();
        gph[u].push(v);
        gph[v].push(u);
        sub_dist[u].push(0);
        sub_dist[v].push(0);
    }
    let mut ans = vec![0; n];
    let mut cnts = vec![0; n];
    dfs0(&gph, 0, n, &mut sub_dist, &mut cnts, &mut ans);
    dfs1(&gph, 0, n, 0, &sub_dist, &cnts, &mut ans);
    for x in ans {
        print!("{} ", x);
    }
    println!("");
    Ok(())
}

fn dfs0(
    gph: &Vec<Vec<usize>>,
    node: usize,
    parent: usize,
    sub_dist: &mut Vec<Vec<i64>>,
    cnts: &mut Vec<i64>,
    ans: &mut Vec<i64>,
) -> (i64, i64) {
    let mut c = 1;
    let mut x = 0;
    for (i, &nxt) in gph[node].iter().enumerate() {
        if nxt != parent {
            let (sub_c, sub_x) = dfs0(gph, nxt, node, sub_dist, cnts, ans);
            sub_dist[node][i] = sub_x + sub_c;
            x += sub_x + sub_c;
            c += sub_c;
        }
    }
    cnts[node] = c;
    ans[node] = x;
    (c, x)
}

fn dfs1(
    gph: &Vec<Vec<usize>>,
    node: usize,
    parent: usize,
    others: i64,
    sub_dist: &Vec<Vec<i64>>,
    cnts: &Vec<i64>,
    ans: &mut Vec<i64>,
) {
    for (i, &nxt) in gph[node].iter().enumerate() {
        if nxt != parent {
            let others = others + cnts[0] - cnts[nxt] + ans[node] - sub_dist[node][i];
            dfs1(gph, nxt, node, others, sub_dist, cnts, ans);
        }
    }
    ans[node] += others;
}
