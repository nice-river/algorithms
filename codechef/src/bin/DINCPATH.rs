#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{
    fmt::Debug,
    io::{BufReader, Read, Write},
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

use std::collections::BTreeMap;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    let input = input.lock();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);
    let writer = std::io::stdout();
    let mut writer = writer.lock();

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let m: usize = reader.read();
        let mut gph = BTreeMap::<i64, Vec<(usize, usize)>>::new();

        let mut values = vec![0i64; n + 1];
        for i in 1..=n {
            values[i] = reader.read();
        }
        for _ in 0..m {
            let u: usize = reader.read();
            let v: usize = reader.read();
            if values[u] < values[v] {
                gph.entry(values[v] - values[u]).or_insert_with(|| vec![]).push((u, v));
            } else if values[u] > values[v] {
                gph.entry(values[u] - values[v]).or_insert_with(|| vec![]).push((v, u));
            }
        }

        let mut dp = vec![0; n + 1];

        for (_, edges) in gph.into_iter() {
            let mut dp_nxt = BTreeMap::new();
            for (u, v) in edges {
                if dp[v] < dp[u] + 1 {
                    dp_nxt.insert(v, (*dp_nxt.get(&v).unwrap_or(&0)).max(dp[u] + 1));
                }
            }
            for (k, v) in dp_nxt.into_iter() {
                dp[k] = v;
            }
        }

        let ans = dp.into_iter().max().unwrap();
        writeln!(writer, "{}", ans + 1)?;
    }

    Ok(())
}
