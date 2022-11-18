#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{
    convert::TryInto,
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

    let n: usize = reader.read();
    let mut dsu = DSU::new(n);

    for _ in 0..reader.read() {
        let u: usize = reader.read();
        let v: usize = reader.read();
        dsu.merge(u, v);
    }
    let mut mp = BTreeMap::new();
    for i in 1..=n {
        let c: i64 = reader.read();
        let p = dsu.find(i);
        if c >= 0 {
            if mp.get(&p).is_none() || *mp.get(&p).unwrap() > c {
                mp.insert(p, c);
            }
        }
    }
    if mp.len() != dsu.set_count() {
        if dsu.set_count() == 1 {
            writeln!(writer, "0")?;
        } else {
            writeln!(writer, "-1")?;
        }
    } else {
        let mut mp = mp.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
        mp.sort();
        let ans = mp[0] * (mp.len() - 1) as i64 + mp.into_iter().skip(1).sum::<i64>();
        writeln!(writer, "{}", ans)?;
    }

    Ok(())
}

struct DSU {
    mark: Vec<usize>,
    disjointed_sets: usize,
}

impl DSU {
    fn new<T: TryInto<usize>>(n: T) -> Self {
        let n = n.try_into().unwrap_or_else(|_| unreachable!());
        Self {
            mark: vec![0; n + 1],
            disjointed_sets: n,
        }
    }

    fn find<T: TryInto<usize> + Copy>(&mut self, x: T) -> usize {
        let p = x.try_into().unwrap_or_else(|_| unreachable!());
        if self.mark[p] == 0 {
            p
        } else {
            let r = self.find(self.mark[p]);
            self.mark[p] = r.try_into().unwrap_or_else(|_| unreachable!());
            r
        }
    }

    fn merge<T: TryInto<usize> + Copy>(&mut self, a: T, b: T) {
        let u = self.find(a);
        let v = self.find(b);
        if u != v {
            self.mark[u] = v;
            self.disjointed_sets -= 1;
        }
    }

    fn set_count(&self) -> usize {
        self.disjointed_sets
    }

    fn is_same_set<T: TryInto<usize> + Copy>(&mut self, a: T, b: T) -> bool {
        self.find(a) == self.find(b)
    }
}
