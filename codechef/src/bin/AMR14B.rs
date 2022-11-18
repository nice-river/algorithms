#![allow(non_snake_case)]
#![allow(dead_code)]

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

struct Graph {
    vertices: Vec<usize>,
    edges: Vec<(usize, usize, i64)>,
    cur_idx: usize,
}

impl Graph {
    fn new(n: usize, m: usize) -> Self {
        let vertices = vec![0; n + 1];
        let edges = vec![(0, 0, 0); 2 * (m + 1)];
        Self {
            vertices,
            edges,
            cur_idx: 0,
        }
    }

    fn push(&mut self, u: usize, v: usize, w: i64) {
        self.cur_idx += 1;
        self.edges[self.cur_idx].0 = self.vertices[u];
        self.edges[self.cur_idx].1 = v;
        self.edges[self.cur_idx].2 = -w;
        self.vertices[u] = self.cur_idx;
    }

    fn reset(&mut self, n: usize) {
        self.vertices[..n].fill(0);
        self.cur_idx = 0;
    }

    fn adj_edges(&self, u: usize) -> AdjEdges {
        AdjEdges::new(self, u)
    }
}

struct AdjEdges<'a> {
    graph: &'a Graph,
    vertex: usize,
    cur_edge_idx: usize,
}

impl<'a> AdjEdges<'a> {
    fn new(graph: &'a Graph, vertex: usize) -> Self {
        Self {
            graph,
            vertex,
            cur_edge_idx: graph.vertices[vertex],
        }
    }
}

impl<'a> Iterator for AdjEdges<'a> {
    type Item = (usize, i64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_edge_idx == 0 {
            None
        } else {
            let (nxt_idx, v, w) = self.graph.edges[self.cur_edge_idx];
            self.cur_edge_idx = nxt_idx;
            Some((v, w))
        }
    }
}

use std::collections::BinaryHeap;

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
        let mut gph = vec![vec![]; n];
        for _ in 0..reader.read() {
            let u: usize = reader.read();
            let v: usize = reader.read();
            let w: i64 = reader.read();
            gph[u].push((v, -w));
            gph[v].push((u, -w));
        }

        let mut que = BinaryHeap::new();
        let mut dist = vec![i64::MIN; n];
        let mut vis = vec![false; n];
        dist[0] = 0;
        que.push((0, 0));
        while let Some((d, cur)) = que.pop() {
            if d < dist[cur] {
                continue;
            }
            for &(nxt, w) in &gph[cur] {
                if dist[nxt] < d + w {
                    dist[nxt] = d + w;
                    que.push((dist[nxt], nxt));
                }
            }
        }
        if dist.iter().any(|&x| x == i64::MIN) {
            writeln!(writer, "NO")?;
            continue;
        }
        let mut que = BinaryHeap::new();
        let mut dp = vec![i64::MIN; n];
        vis.fill(false);
        que.push((0, 0, 0));
        dp[0] = 0;
        let mut ans = true;
        while let Some((_, b, cur)) = que.pop() {
            if b < dist[cur] && !vis[cur] {
                ans = false;
                break;
            }
            if vis[cur] {
                continue;
            }
            vis[cur] = true;
            dp[cur] = b;
            for &(nxt, w) in &gph[cur] {
                if dp[nxt] < b + w {
                    que.push((w, b + w, nxt));
                }
            }
        }
        if ans {
            if dp.iter().any(|&x| x == i64::MIN) {
                writeln!(writer, "NO")?;
            } else {
                writeln!(writer, "YES")?;
            }
        } else {
            writeln!(writer, "NO")?;
        }
    }

    Ok(())
}
