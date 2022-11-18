#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{
    collections::VecDeque,
    fmt::Debug,
    io::{self, BufReader, Read, Write},
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

use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    let input = input.lock();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);
    let writer = io::stdout();
    let mut writer = writer.lock();

    'Test: for _ in 0..1 {
        let n: usize = reader.read();
        let e: usize = reader.read();
        let mut gph = vec![VecDeque::new(); n + 1];
        let mut roads = Vec::with_capacity(e + 1);
        for i in 0..e {
            let u: usize = reader.read();
            let v: usize = reader.read();
            gph[u].push_back((i, v));
            gph[v].push_back((i, u));
            roads.push((u, v));
        }

        if e < n {
            println!("NO");
            continue 'Test;
        }

        for r in &gph {
            if r.len() % 2 != 0 {
                println!("NO");
                continue 'Test;
            }
        }
        let mut vis = vec![false; e];
        let mut ans = HashSet::with_capacity(e + 1);
        let mut path = Vec::with_capacity(e + 1);
        dfs(&mut gph, 1, &mut vis, &mut path, &mut ans);

        if ans.len() < roads.len() {
            println!("NO");
            continue 'Test;
        }

        println!("YES");
        for road in roads {
            if ans.contains(&road) {
                writeln!(writer, "{} {}", road.0, road.1)?;
            } else {
                writeln!(writer, "{} {}", road.1, road.0)?;
            }
        }
    }

    Ok(())
}

fn dfs(
    gph: &mut Vec<VecDeque<(usize, usize)>>,
    node: usize,
    vis: &mut Vec<bool>,
    path: &mut Vec<usize>,
    ans: &mut HashSet<(usize, usize)>,
) {
    while !gph[node].is_empty() {
        let (i, nxt) = gph[node].pop_front().unwrap();
        if !vis[i] {
            vis[i] = true;
            if path.is_empty() {
                path.push(node);
            }
            path.push(nxt);
            dfs(gph, nxt, vis, path, ans);
        }
    }

    if !path.is_empty() {
        for i in 1..path.len() {
            ans.insert((path[i - 1], path[i]));
        }
        path.clear();
    }
}
