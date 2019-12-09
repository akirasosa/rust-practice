#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;

#[allow(unused_macros)]
macro_rules! debug {
    ($($e:expr),*) => {
        #[cfg(debug_assertions)]
        $({
            let (e, mut err) = (stringify!($e), std::io::stderr());
            writeln!(err, "{} = {:?}", e, $e).unwrap()
        })*
    };
}

pub fn div_euclid(a: i64, rhs: i64) -> i64 {
    let q = a / rhs;
    if a % rhs < 0 {
        return if rhs > 0 { q - 1 } else { q + 1 };
    }
    q
}

fn main() {
    println!("{:#010b}", (1 << 5) - 1)
}
