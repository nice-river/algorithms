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

use std::collections::HashSet;

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
        let mut a = vec![0; n + 1];
        let mut b = vec![0; n + 1];
        for i in 1..=n {
            a[i] = reader.read();
        }
        for i in 1..=n {
            b[i] = reader.read();
        }
        let mut sels = HashSet::new();
        let mut mini = Fraction {
            numerator: 0,
            denominator: 1,
        };
        for (i, f) in a
            .into_iter()
            .zip(b.into_iter())
            .skip(1)
            .map(|(numerator, denominator)| Fraction {
                numerator,
                denominator,
            })
            .enumerate()
        {
            if mini < f {
                sels.clear();
                sels.insert(i + 1);
                mini = f;
            } else if mini == f {
                sels.insert(i + 1);
            }
        }
        let mut gph = vec![vec![]; n + 1];
        let mut vis = vec![false; n + 1];
        for _ in 0..m {
            let u: usize = reader.read();
            let v: usize = reader.read();
            if sels.contains(&u) && sels.contains(&v) {
                gph[u].push(v);
                gph[v].push(u);
            }
        }
        let mut ans = vec![];
        let mut tmp = vec![];
        for i in 1..=n {
            if !vis[i] {
                tmp.clear();
                dfs(&gph, &mut vis, &mut tmp, i);
                if tmp.len() > ans.len() {
                    ans = tmp.clone();
                }
            }
        }
        writeln!(writer, "{}", ans.len())?;
        for x in ans {
            write!(writer, "{} ", x)?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

fn dfs(gph: &Vec<Vec<usize>>, vis: &mut Vec<bool>, path: &mut Vec<usize>, cur: usize) {
    path.push(cur);
    vis[cur] = true;
    for &nxt in &gph[cur] {
        if !vis[nxt] {
            dfs(gph, vis, path, nxt);
        }
    }
}

struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.denominator * other.numerator == self.numerator * other.denominator
    }
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.numerator * other.denominator).cmp(&(self.denominator * other.numerator))
    }
}
