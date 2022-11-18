#![allow(non_snake_case, unused_imports, unused_variables, dead_code)]

use std::{
    fmt::Debug,
    fs::read,
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

use std::collections::BinaryHeap;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    let input = input.lock();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);
    let writer = std::io::stdout();
    let mut writer = writer.lock();

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
    enum Mode {
        Up = 0,
        Down = 1,
    }

    let n: usize = reader.read();
    let m: usize = reader.read();
    let mut gph = vec![vec![]; n + 1];

    let mut height = vec![0; n + 1];
    let mut charge = vec![0; n + 1];

    for i in 1..=n {
        height[i] = reader.read::<i32>();
    }
    for i in 1..=n {
        charge[i] = -reader.read::<i64>();
    }

    for _ in 0..m {
        let u: usize = reader.read();
        let v: usize = reader.read();
        gph[u].push(v);
        gph[v].push(u);
    }

    let mut que = BinaryHeap::new();
    let mut vis = vec![vec![false; 2]; n + 1];
    vis[1][Mode::Down as usize] = true;
    vis[1][Mode::Up as usize] = true;

    for &nxt in &gph[1] {
        match height[1].cmp(&height[nxt]) {
            std::cmp::Ordering::Less => {
                que.push((charge[1], nxt, Mode::Up));
            }
            std::cmp::Ordering::Equal => {
                que.push((charge[1], nxt, Mode::Up));
                que.push((charge[1], nxt, Mode::Down));
            }
            std::cmp::Ordering::Greater => {
                que.push((charge[1], nxt, Mode::Down));
            }
        }
    }

    let mut ans = -1;
    while let Some((c, cur, mode)) = que.pop() {
        if cur == n {
            ans = -c;
            break;
        }
        if vis[cur][mode as usize] {
            continue;
        }
        vis[cur][mode as usize] = true;
        for &nxt in &gph[cur] {
            match height[cur].cmp(&height[nxt]) {
                std::cmp::Ordering::Less => match mode {
                    Mode::Up => {
                        que.push((c, nxt, mode));
                    }
                    Mode::Down => que.push((c + charge[cur], nxt, Mode::Up)),
                },
                std::cmp::Ordering::Equal => {
                    que.push((c, nxt, mode));
                    match mode {
                        Mode::Up => que.push((c + charge[cur], nxt, Mode::Down)),
                        Mode::Down => que.push((c + charge[cur], nxt, Mode::Up)),
                    }
                }
                std::cmp::Ordering::Greater => match mode {
                    Mode::Up => que.push((c + charge[cur], nxt, Mode::Down)),
                    Mode::Down => {
                        que.push((c, nxt, mode));
                    }
                },
            }
        }
    }
    writeln!(writer, "{}", ans)?;

    Ok(())
}
