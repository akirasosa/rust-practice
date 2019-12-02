use std::cmp;
use std::cmp::{max, min};
use std::cmp::Ordering::{self, Greater, Less};
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
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

pub trait Ext {
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
        (base.0 + (cmp.0 == Less) as usize..base.1 + (cmp.1 != Greater) as usize)
    }

    fn equal_range_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> std::ops::Range<usize>
        where
            F: FnMut(&'a Self::Item) -> K,
            K: Ord,
    {
        self.equal_range_by(|e| f(e).cmp(k))
    }
}

pub struct UnionFind {
    parts: Vec<usize>
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let parts = (0..n).collect();
        UnionFind { parts: parts }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let i_leader = self.find(i);
        let j_leader = self.find(j);
        self.parts[j_leader] = self.parts[i_leader];
    }

    pub fn find(&mut self, i: usize) -> usize {
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

    pub fn find_only(&self, i: usize) -> usize {
        let mut p = i;
        while self.parts[p] != p {
            p = self.parts[p]
        };
        p
    }

    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    pub fn same_only(&self, i: usize, j: usize) -> bool {
        self.find_only(i) == self.find_only(j)
    }
}

pub struct BinaryIndexTree<T> {
    data: Vec<T>,
}

impl<T> BinaryIndexTree<T>
    where
        T: Copy + AddAssign + Sub<Output=T> + Default,
{
    pub fn new(size: usize) -> Self {
        let buf_size = size.next_power_of_two();
        BinaryIndexTree {
            data: vec![T::default(); buf_size + 1],
        }
    }

    pub fn range_sum(&self, l: usize, r: usize) -> T {
        self.sum(r - 1) - self.sum(l - 1)
    }

    pub fn sum(&self, i: usize) -> T {
        let mut i = i as i64;
        let mut ret = T::default();
        while i > 0 {
            ret += self.data[i as usize];
            i -= i & -i;
        }
        ret
    }

    pub fn add(&mut self, i: usize, value: T) {
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

#[allow(dead_code)]
fn codefestival_2015_qual_a_d() {
    input! {
        n: i64,
        m: i64,
        arr: [i64; m],
    }
    let n: i64 = n;
    let arr: Vec<i64> = arr;

    let query = |t: i64| {
        let mut boundary = 0;

        for &a in &arr {
            let mut dist = a - boundary - 1;
            if dist < 0 {
                dist = 0;
            }
            if dist > t {
                return false;
            }
            let b0 = max(boundary, a + t - dist * 2);
            let b1 = a + (t - dist) / 2;
            boundary = max(b0, b1);
        }

        return boundary >= n;
    };

    let l = 0;
    let r = n * 2;
    let mut size = r - l + 1;
    let mut base = l;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        if !query(mid) {
            base = mid;
        }
        size -= half;
    }

    println!("{}", base + !query(base) as i64);
}

#[allow(dead_code)]
fn abc034_d() {
    input! {
        n: usize,
        k: usize,
        arr: [[f64; 2]; n],
    }
    let k: usize = k;
    let arr: Vec<Vec<f64>> = arr;

    let query = |x: f64| {
        let mut items: Vec<f64> = arr.iter()
            .map(|a| {
                a[0] * (a[1] - x)
            })
            .collect();
        items.sort_by(|a, b| b.partial_cmp(a).unwrap());
        items[..k].iter().sum::<f64>() >= 0.
    };

    let l = 0.;
    let r = 100.;
    let mut size = r - l;
    let mut base = l;
    while size > 1e-6 {
        let half = size / 2.;
        let mid = base + half;
        if query(mid) {
            base = mid;
        }
        size -= half;
    }

    println!("{}", base);
}

#[allow(dead_code)]
fn arc026_4() {
    input! {
        n: usize,
        m: usize,
        arr: [(usize, usize, f64, f64); m],
    }
    let n: usize = n;
    let arr: Vec<(usize, usize, f64, f64)> = arr;

    struct Edge {
        from: usize,
        to: usize,
        cost: f64,
        time: f64,
    }
    impl Edge {
        fn cost_at(&self, x: f64) -> f64 {
            self.cost - x * self.time
        }
    }

    let mut edges: Vec<Edge> = arr.iter()
        .map(|a| {
            Edge {
                from: a.0,
                to: a.1,
                cost: a.2,
                time: a.3,
            }
        })
        .collect();

    let max_salary = edges.iter()
        .map(|e| {
            e.cost / e.time
        })
        .fold(0., |acc, a| {
            if a > acc {
                a
            } else {
                acc
            }
        });

    let mut query = |x: f64| {
        edges.sort_by(|a, b| {
            let cost_a = a.cost_at(x);
            let cost_b = b.cost_at(x);
            cost_a.partial_cmp(&cost_b).unwrap()
        });

        let mut balance = 0.;
        let mut parity = UnionFind::new(n);

        for e in &edges {
            let cost = e.cost_at(x);
            if !parity.same(e.from, e.to) || cost < 0. {
                parity.union(e.from, e.to);
                balance += cost;
            }
        }

        balance <= 0.
    };

    let l = 0.;
    let r = max_salary;
    let mut size = r - l;
    let mut base = l;
    while size > 1e-3 {
        let half = size / 2.;
        let mid = base + half;
        if !query(mid) {
            base = mid;
        }
        size -= half;
    }
    println!("{}", base);
}

#[allow(dead_code)]
fn arc060_a() {
    input! {
        n: usize,
        avg: i64,
        arr: [i64; n],
    }
    let n: usize = n;
    let avg: i64 = avg;
    let arr: Vec<i64> = arr;

    // left index, sum, number of cards
    type Args = (usize, i64, usize);

    struct Solver {
        arr: Vec<i64>,
        cache: HashMap<Args, usize>,
    }
    impl Solver {
        fn solve(&mut self, args: Args) -> usize {
            if self.cache.contains_key(&args) {
                return *self.cache.get(&args).unwrap();
            }
            let res = self.inner(args);
            self.cache.insert(args, res);
            res
        }

        fn inner(&mut self, args: Args) -> usize {
            let (i, j, k) = args;
            if i == 0 && j == 0 && k == 0 {
                return 1;
            }
            if i >= 1 && j < self.arr[i - 1] {
                return self.solve((i - 1, j, k));
            }
            if i >= 1 && k >= 1 && j >= self.arr[i - 1] {
                let last = self.arr[i - 1];
                let n0 = self.solve((i - 1, j - last, k - 1));
                let n1 = self.solve((i - 1, j, k));
                return n0 + n1;
            }
            0
        }
    }

    let cache = HashMap::new();
    let mut solver = Solver { arr: arr, cache: cache };
    let res: usize = (1..n + 1)
        .map(|m| {
            solver.solve((n, avg * m as i64, m))
        })
        .sum();
    println!("{:?}", res);
}

//#[allow(dead_code)]
//fn practice_dp0() {
////    let n: usize = 3;
////    let arr = vec![7, 5, 3];
////    let total = 10;
//    let n: usize = 2;
//    let arr = vec![9, 7];
//    let total = 6;
//
//    let mut dp = vec![vec![false; total + 1]; n + 1];
//    dp[0][0] = true;
//
//    for i in 0..n {
//        for j in 0..total + 1 {
//            if j >= arr[i] {
//                dp[i + 1][j] = dp[i][j - arr[i]] || dp[i][j];
//            } else {
//                dp[i + 1][j] = dp[i][j];
//            }
//        }
//    }
//
//    println!("{:?}", dp);
//    println!("{:?}", dp[n][total]);
//}

#[allow(dead_code)]
fn arc075_c() {
    input! {
        n: usize,
        k: i64,
        aa: [i64; n],
    }
    let k: i64 = k;
    let aa: Vec<i64> = aa;

    let aa = aa.iter()
        .scan(0i64, |state, a| {
            *state += a - k;
            Some(*state)
        })
        .collect();
    let aa = [vec![0], aa].concat();
    let aa = coord_compress(aa);

    let mut bit = BinaryIndexTree::new(aa.len());
    let res = aa.iter()
        .fold(0usize, |acc, &x| {
            let acc = acc + bit.sum(x + 1);
            bit.add(x + 1, 1);
            acc
        });

    println!("{:?}", res);
}

#[allow(dead_code)]
fn abc032_c() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n],
    }
    let n: usize = n;
    let k: usize = k;
    let aa: Vec<usize> = aa;

    let mut res = 0;
    let mut r = 0;
    let mut prod = 1;

    for l in 0..n {
        while r < n && prod * aa[r] <= k {
            if aa[r] == 0 {
                println!("{}", n);
                return;
            }
            prod *= aa[r];
            r += 1;
        }

        if r - l > res {
            res = r - l;
        }

        if r == l {
            r += 1;
        } else {
            prod /= aa[l];
        }
    }

    println!("{}", res);
}

#[allow(dead_code)]
fn abc038_c() {
    input! {
        n: usize,
        aa: [usize; n],
    }
    let n: usize = n;
    let aa: Vec<usize> = aa;

    let mut res = 0;
    let mut r = 0;
    let mut is_mi = false;

    for l in 0..n {
        while r < n && (r == l || (is_mi && (aa[r - 1] < aa[r]))) {
            is_mi = true;
            r += 1;
        }

        res += r - l;

        if r == l {
            r += 1;
        } else {
            is_mi = l == n - 1 || is_mi && (aa[l] < aa[l + 1]);
        }
    }

    println!("{}", res);
}

#[allow(dead_code)]
fn arc098_b() {
    input! {
        n: usize,
        aa: [usize; n],
    }
    let n: usize = n;
    let aa: Vec<usize> = aa;

    let mut res = 0;
    let mut r = 0;
    let mut sum = 0;
    let mut xor = 0;

    for l in 0..n {
        while r < n && (xor ^ aa[r] == sum + aa[r]) {
            xor ^= aa[r];
            sum += aa[r];
            r += 1;
        }

        res += r - l;

        if r == l {
            r += 1;
        } else {
            xor ^= aa[l];
            sum -= aa[l];
        }
    }

    println!("{}", res);
}

#[allow(dead_code)]
fn arc022_2() {
    input! {
        n: usize,
        aa: [usize; n],
    }
    let n: usize = n;
    let aa: Vec<usize> = aa;

    let mut res = 0;
    let mut r = 0;
    let mut state: HashSet<usize> = HashSet::new();

    for l in 0..n {
        while r < n && !state.contains(&aa[r]) {
            state.insert(aa[r]);
            r += 1;
        }

        res = max(r - l, res);

        if r == l {
            r += 1;
        } else {
            state.remove(&aa[l]);
        }
    }

    println!("{}", res);
}

#[allow(dead_code)]
fn abc017_4() {
    input! {
        n: usize,
        aa: [usize; n],
    }
    let n: usize = n;
    let aa: Vec<usize> = aa;

    let mut r = 0;
    let mut state: HashSet<usize> = HashSet::new();
    let mut bi = BinaryIndexTree::new(n + 1);
    let mut res = 0;
    bi.add(1, 1usize);

    for l in 0..n {
        while r < n && !state.contains(&aa[r]) {
            state.insert(aa[r]);
            r += 1;

            let s = bi.range_sum(l + 1, r + 1) % 1000000007;
            bi.add(r + 1, s);
            res = s;
        }

        if r == l {
            r += 1;
        } else {
            state.remove(&aa[l]);
        }
    }

    println!("{}", res);
}


#[allow(dead_code)]
fn abc033_d() {
    input! {
        n: usize,
        aa: [(i64, i64); n],
    }
    let n: usize = n;
    let aa: Vec<(i64, i64)> = aa;

    fn prod(a: (i64, i64), b: (i64, i64)) -> i64 {
        a.0 * b.0 + a.1 * b.1
    }

    fn is_lte_90(a: (f64, (i64, i64)), b: (f64, (i64, i64))) -> bool {
        let (rad_a, vec_a) = a;
        let (rad_b, vec_b) = b;
        rad_b - rad_a < PI && prod(vec_a, vec_b) >= 0
    };

    let count = |i: usize| {
        let o = &aa[i];
        let mut rads_vecs: Vec<(f64, (i64, i64))> = aa.iter()
            .filter(|&a| a != o)
            .map(|a| {
                let &(x0, y0) = o;
                let &(x1, y1) = a;
                let rad = ((y1 - y0) as f64).atan2((x1 - x0) as f64);
                let v = (x1 - x0, y1 - y0);
                if rad < 0.0 {
                    (2. * PI + rad, v)
                } else {
                    (rad, v)
                }
            })
            .collect();
        rads_vecs.sort_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap()
        });
        let rads_vecs: Vec<(f64, (i64, i64))> = rads_vecs.iter()
            .map(|&(rad, vec)| {
                (rad - 2. * PI, vec)
            })
            .chain(rads_vecs.iter().cloned())
            .collect();

        let n = rads_vecs.len();
        let mut r = 0;
        let mut n_lt_90 = 0;
        let mut n_eq_90 = 0;

        for l in 0..(n / 2) {
            while r < n && is_lte_90(rads_vecs[l], rads_vecs[r]) {
                r += 1;
            }

            let vec_a = rads_vecs[l].1;
            let vec_b = rads_vecs[r - 1].1;
            if prod(vec_a, vec_b) == 0 {
                n_eq_90 += 1;
                n_lt_90 += r - l - 2;
            } else {
                n_lt_90 += r - l - 1;
            }

            if r == l {
                r += 1;
            }
        }

        let n_gt_90 = (n / 2) * (n / 2 - 1) / 2 - n_lt_90 - n_eq_90;
        (n_lt_90, n_eq_90, n_gt_90)
    };

    let mut n_eq_90 = 0;
    let mut n_gt_90 = 0;
    for i in 0..n {
        let cnt = count(i);
        n_eq_90 += cnt.1;
        n_gt_90 += cnt.2;
    }
    let n_lt_90 = n * (n - 1) * (n - 2) / 6 - n_eq_90 - n_gt_90;

    println!("{} {} {}", n_lt_90, n_eq_90, n_gt_90);
}

#[allow(dead_code)]
fn arc064_a() {
    input! {
        n: usize,
        x: usize,
        aa: [usize; n],
    }
    let n: usize = n;
    let x: usize = x;
    let aa: Vec<usize> = [vec![0], aa].concat();
    let mut bb: Vec<usize> = vec![0; n + 1];

    for i in 0..n {
        let xi = x - (aa[i] - bb[i]);
        bb[i + 1] = if aa[i + 1] > xi {
            aa[i + 1] - xi
        } else {
            0
        }
    }

    println!("{}", bb.iter().sum::<usize>());
}

fn main() {
    arc064_a();
}

