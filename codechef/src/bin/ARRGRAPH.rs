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
            buf: vec![0; 128],
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
    let input = input.lock();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);
    let writer = std::io::stdout();
    let mut writer = writer.lock();

    let mut coprimes = vec![vec![false; 51]; 51];
    for a in 2..=50 {
        for b in a + 1..=50 {
            if gcd(a, b) == 1 {
                coprimes[a][b] = true;
                coprimes[b][a] = true;
            }
        }
    }

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let mut arr = Vec::with_capacity(n);
        for _ in 0..n {
            arr.push(reader.read::<usize>());
        }
        let mut dsu = DSU::new(n);
        let mut p = n;
        for i in 0..n {
            for j in i + 1..n {
                if coprimes[arr[i]][arr[j]] {
                    dsu.merge(i, j);
                }
            }
            if arr[i] == 47 {
                p = i;
            }
        }
        if dsu.set_count() != 1 {
            if p == n {
                arr[0] = 47;
            } else {
                arr[p] = 46;
            }
            writeln!(writer, "1")?;
        } else {
            writeln!(writer, "0")?;
        }
        for v in arr {
            write!(writer, "{} ", v)?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
