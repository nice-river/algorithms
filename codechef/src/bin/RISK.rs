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

static DIRS: [i32; 5] = [-1, 0, 1, 0, -1];

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let m: usize = reader.read();
        let mut grid = vec![vec![0; m]; n];
        for i in 0..n {
            let s: String = reader.read();
            let mut s = s.into_bytes();
            s.iter_mut().for_each(|e| *e = *e - b'0');
            grid[i] = s;
        }
        println!("{}", solve(grid));
    }

    Ok(())
}

fn solve(grid: Vec<Vec<u8>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut vis = vec![vec![false; m]; n];

    let mut areas = vec![];

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 && !vis[i][j] {
                vis[i][j] = true;
                areas.push(dfs(&grid, &mut vis, i, j));
            }
        }
    }

    areas.sort();
    areas.iter().rev().skip(1).step_by(2).sum::<i32>()
}

fn dfs(grid: &Vec<Vec<u8>>, vis: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i32 {
    let mut ans = 1;
    for d in 0..4 {
        let x = i as i32 + DIRS[d];
        let y = j as i32 + DIRS[d + 1];
        if x < 0 || y < 0 {
            continue;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= grid.len() || y >= grid[0].len() || vis[x][y] || grid[x][y] == 0 {
            continue;
        }
        vis[x][y] = true;
        ans += dfs(grid, vis, x, y);
    }
    ans
}
