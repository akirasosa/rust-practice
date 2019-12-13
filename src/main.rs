use std::borrow::Borrow;
use std::cmp::{max, min};
use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::io::Write;
use std::ops::{AddAssign, Deref, Sub};

enum Ops {
    Sum,
    Max,
}

struct BinaryIndexTree<T> {
    data: Vec<T>,
    ops: Ops,
}

impl<T> BinaryIndexTree<T>
    where
        T: Copy + AddAssign + Sub<Output=T> + Default + Ord,
{
    fn new(size: usize, ops: Ops) -> Self {
        let buf_size = size.next_power_of_two();
        BinaryIndexTree {
            data: vec![T::default(); buf_size + 1],
            ops: ops,
        }
    }

    fn query(&self, i: usize) -> T {
        let mut i = i as i64;
        let mut ret = T::default();
        match self.ops {
            Ops::Sum => {
                while i > 0 {
                    ret += self.data[i as usize];
                    i -= i & -i;
                }
            }
            Ops::Max => {
                while i > 0 {
                    ret = max(ret, self.data[i as usize]);
                    i -= i & -i;
                }
            }
        }
        ret
    }

    fn set(&mut self, i: usize, value: T) {
        assert!(i > 0);
        let n = self.data.len() as i64;
        let mut i = i as i64;
        match self.ops {
            Ops::Sum => {
                while i <= n - 1 {
                    self.data[i as usize] += value;
                    i += i & -i;
                }
            }
            Ops::Max => {
                while i <= n - 1 && self.data[i as usize] < value {
                    self.data[i as usize] = value;
                    i += i & -i;
                }
            }
        }
    }
}

struct BIT {
    v: Vec<i64>,
    n: usize,
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT { v: vec![0; n + 1], n: n }
    }

    // update i th element by x
    fn update(&mut self, i: usize, x: i64) {
        let mut id = i as i64 + 1;
        while id <= self.n as i64 && self.v[id as usize] < x {
            self.v[id as usize] = x;
            id += id & -id;
        }
    }

    // max in 0 to i th element
    fn max(&self, i: usize) -> i64 {
        let mut id = i + 1;
        let mut ret = i64::min_value();
        while id > 0 {
            ret = std::cmp::max(ret, self.v[id]);
            id &= id - 1;
        }
        ret
    }
}

fn main() {
    let aa = vec![1, 4, 5];
    println!("{:?}", aa[0..1].to_vec())
}
