#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(non_snake_case)]

use std::cmp::{max, min};
use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::io::Write;
use std::ops::{AddAssign, Sub};

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
}

trait Ext {
    type Item;

    fn lower_bound(&self, x: &Self::Item) -> usize
        where
            Self::Item: Ord;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
        where
            F: FnMut(&'a Self::Item) -> Ordering;

    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> usize
        where
            F: FnMut(&'a Self::Item) -> K,
            K: Ord;

    fn upper_bound(&self, x: &Self::Item) -> usize
        where
            Self::Item: Ord;

    fn upper_bound_by<'a, F>(&'a self, f: F) -> usize
        where
            F: FnMut(&'a Self::Item) -> Ordering;

    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> usize
        where
            F: FnMut(&'a Self::Item) -> K,
            K: Ord;

    fn equal_range(&self, x: &Self::Item) -> std::ops::Range<usize>
        where
            Self::Item: Ord;

    fn equal_range_by<'a, F>(&'a self, f: F) -> std::ops::Range<usize>
        where
            F: FnMut(&'a Self::Item) -> Ordering;

    fn equal_range_by_key<'a, K, F>(&'a self, k: &K, f: F) -> std::ops::Range<usize>
        where
            F: FnMut(&'a Self::Item) -> K,
            K: Ord;
}

impl<T> Ext for [T] {
    type Item = T;

    fn lower_bound(&self, x: &Self::Item) -> usize
        where
            T: Ord,
    {
        self.lower_bound_by(|y| y.cmp(x))
    }
    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
        where
            F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp == Less) as usize
    }
    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> usize
        where
            F: FnMut(&'a Self::Item) -> K,
            K: Ord,
    {
        self.lower_bound_by(|e| f(e).cmp(k))
    }

    fn upper_bound(&self, x: &Self::Item) -> usize
        where
            T: Ord,
    {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> usize
        where
            F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { s.get_unchecked(mid) });
            base = if cmp == Greater { base } else { mid };
            size -= half;
        }
        let cmp = f(unsafe { s.get_unchecked(base) });
        base + (cmp != Greater) as usize
    }
    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> usize
        where
            F: FnMut(&'a Self::Item) -> K,
            K: Ord,
    {
        self.upper_bound_by(|e| f(e).cmp(k))
    }

    fn equal_range(&self, x: &Self::Item) -> std::ops::Range<usize>
        where
            T: Ord,
    {
        self.equal_range_by(|y| y.cmp(x))
    }
    fn equal_range_by<'a, F>(&'a self, mut f: F) -> std::ops::Range<usize>
        where
            F: FnMut(&'a Self::Item) -> Ordering,
    {
        let s = self;
        let mut size = s.len();
        if size == 0 {
            return 0..0;
        }
        let mut base = (0usize, 0usize);
        while size > 1 {
            let half = size / 2;
            let mid = (base.0 + half, base.1 + half);
            let cmp = (
                f(unsafe { s.get_unchecked(mid.0) }),
                f(unsafe { s.get_unchecked(mid.1) }),
            );
            base = (
                if cmp.0 == Less { mid.0 } else { base.0 },
                if cmp.1 == Greater { base.1 } else { mid.1 },
            );
            size -= half;
        }
        let cmp = (
            f(unsafe { s.get_unchecked(base.0) }),
            f(unsafe { s.get_unchecked(base.1) }),
        );
        base.0 + (cmp.0 == Less) as usize..base.1 + (cmp.1 != Greater) as usize
    }

    fn equal_range_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> std::ops::Range<usize>
        where
            F: FnMut(&'a Self::Item) -> K,
            K: Ord,
    {
        self.equal_range_by(|e| f(e).cmp(k))
    }
}

struct UnionFind {
    parts: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let parts = (0..n).collect();
        UnionFind { parts: parts }
    }

    fn union(&mut self, i: usize, j: usize) {
        let i_leader = self.find(i);
        let j_leader = self.find(j);
        self.parts[j_leader] = self.parts[i_leader];
    }

    fn find(&mut self, i: usize) -> usize {
        let mut p = i;
        while self.parts[p] != p {
            p = self.parts[p]
        };
        let mut s = i;
        while s != p {
            let t = self.parts[s];
            self.parts[s] = p;
            s = t
        };
        p
    }

    fn find_only(&self, i: usize) -> usize {
        let mut p = i;
        while self.parts[p] != p {
            p = self.parts[p]
        };
        p
    }

    fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    fn same_only(&self, i: usize, j: usize) -> bool {
        self.find_only(i) == self.find_only(j)
    }
}

struct BinaryIndexTree<T> {
    data: Vec<T>,
}

impl<T> BinaryIndexTree<T>
    where
        T: Copy + AddAssign + Sub<Output=T> + Default,
{
    fn new(size: usize) -> Self {
        let buf_size = size.next_power_of_two();
        BinaryIndexTree {
            data: vec![T::default(); buf_size + 1],
        }
    }

    fn range_sum(&self, l: usize, r: usize) -> T {
        self.sum(r - 1) - self.sum(l - 1)
    }

    fn sum(&self, i: usize) -> T {
        let mut i = i as i64;
        let mut ret = T::default();
        while i > 0 {
            ret += self.data[i as usize];
            i -= i & -i;
        }
        ret
    }

    fn add(&mut self, i: usize, value: T) {
        assert!(i > 0);
        let n = self.data.len() as i64;
        let mut i = i as i64;
        while i <= n - 1 {
            self.data[i as usize] += value;
            i += i & -i;
        }
    }
}

#[inline]
fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

#[inline]
fn coord_compress<T: std::clone::Clone + std::cmp::Ord>(src: Vec<T>) -> Vec<usize> {
    let mut tmp = src.clone();
    tmp.sort();
    tmp.dedup();
    src.iter()
        .map(|x| {
            tmp.binary_search(x).unwrap()
        })
        .collect()
}

#[inline]
fn binary_search<F>(l: i64, r: i64, query_fn: F) -> i64 where F: Fn(i64) -> bool {
    let mut size = r - l;
    let mut base = l;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        if !query_fn(mid) {
            base = mid;
        }
        size -= half;
    }
    base + !query_fn(base) as i64
}

#[inline]
fn rel<T: PartialOrd + Default>(n: T) -> T {
    if n < T::default() {
        T::default()
    } else {
        n
    }
}

#[inline]
fn rem_euclid(a: i64, rhs: i64) -> i64 {
    let r = a % rhs;
    if r < 0 {
        if rhs < 0 {
            r - rhs
        } else {
            r + rhs
        }
    } else {
        r
    }
}

#[inline]
fn div_euclid(a: i64, rhs: i64) -> i64 {
    let q = a / rhs;
    if a % rhs < 0 {
        return if rhs > 0 { q - 1 } else { q + 1 };
    }
    q
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    input! {
        H: usize,
        W: usize,
        board: [chars; H],
    }
    let H: usize = H;
    let W: usize = W;
    let board: Vec<Vec<char>> = board;

    let mut res = 0;
    let mut dist: HashMap<(i32, i32), usize> = HashMap::new();
    let mut queue = VecDeque::new();

    for i in 0..H {
        for j in 0..W {
            if board[i][j] == '#' {
                let pos = (i as i32, j as i32);
                dist.insert(pos, 0);
                queue.push_back(pos);
            }
        }
    }

    while let Some(src) = queue.pop_front() {
        for &d in DIRECTIONS.iter() {
            let dst = (src.0 + d.0, src.1 + d.1);
            if dst.0 < 0 || dst.0 >= H as i32 { continue; }
            if dst.1 < 0 || dst.1 >= W as i32 { continue; }
            if dist.contains_key(&dst) { continue; }

            let d = dist[&src] + 1;
            dist.insert(dst, d);
            queue.push_back(dst);
            res = d;
        }
    }

    println!("{}", res);
}

