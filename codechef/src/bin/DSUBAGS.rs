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

use std::convert::TryInto;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);

    let n: usize = reader.read();
    let mut dsu = DSU::new(n);

    for _ in 0..reader.read() {
        let t: i32 = reader.read();
        match t {
            1 => {
                let a: usize = reader.read();
                let b: usize = reader.read();
                dsu.merge(a, b);
            }
            2 => {
                let a: usize = reader.read();
                let b: usize = reader.read();
                if dsu.is_same_set(a, b) {
                    println!("YES");
                } else {
                    println!("NO");
                }
            }
            3 => {
                println!("{}", dsu.set_count());
            }
            _ => unreachable!(),
        }
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
