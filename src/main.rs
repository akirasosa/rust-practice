use std::cmp;
use std::collections::HashSet;

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

#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}


#[allow(dead_code)]
fn practice_1() {
    input! {
    n: i32,
    m: [i32; 2],
    s: chars,
    }
    let s: Vec<char> = s;
    let s: String = s.into_iter().collect();

    println!("{} {}", n + m[0] + m[1], s);
}

#[allow(dead_code)]
fn abc086_a() {
    input! {
    n: [i32; 2],
    }
    let prod: i32 = n[0] * n[1];
    let res = if prod % 2 == 0 {
        "Even"
    } else {
        "Odd"
    };
    println!("{}", res)
}

#[allow(dead_code)]
fn abc081_a() {
    input! {
    s: chars,
    }
    let s: Vec<char> = s;
    let res: u32 = s.iter().map(|n| n.to_digit(10).unwrap()).sum();

    println!("{}", res)
}

#[allow(dead_code)]
fn abc081_b() {
    input! {
    n: i32,
    arr: [i32; n],
    }
    let mut arr: Vec<i32> = arr;
    let mut cnt = 0;

    loop {
        if arr.iter().any(|v| (v % 2) == 1) {
            break;
        }
        arr = arr.iter().map(|v| v / 2).collect();
        cnt += 1;
    }

    println!("{}", cnt)
}

#[allow(dead_code)]
fn abc083_b() {
    input! {
    arr: [i32; 3],
    }

    let n: i32 = arr[0];
    let a: i32 = arr[1];
    let b: i32 = arr[2];

    let (vals, _total): (Vec<i32>, Vec<i32>) = (0..n + 1)
        .map(|i| {
            let d0 = i - (i / 10) * 10;
            let d1 = (i / 10) - (i / 100) * 10;
            let d2 = (i / 100) - (i / 1000) * 10;
            let d3 = (i / 1000) - (i / 10000) * 10;
            let d4 = (i / 10000) - (i / 100000) * 10;
            (i, d0 + d1 + d2 + d3 + d4)
        })
        .filter(|&(_i, total)| {
            (a <= total) && (total <= b)
        })
        .unzip();
//        .for_each(|(i, _total)| {
//            res += i;
//        });
    let res: i32 = vals.iter().sum();

    println!("{:?}", res)
}

#[allow(dead_code)]
fn abc087_b() {
    input! {
    n_500: i32,
    n_100: i32,
    n_50: i32,
    total: i32,
    }

    let n_500: i32 = n_500;
    let n_100: i32 = n_100;
    let n_50: i32 = n_50;
    let total: i32 = total;
    let mut cnt = 0;

    for i in 0..n_500 + 1 {
        for j in 0..n_100 + 1 {
            for k in 0..n_50 + 1 {
                let total_tried = i * 500 + j * 100 + k * 50;
                if total_tried == total {
//                    println!("{}, {}, {}", i, j, k);
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt)
}

#[allow(dead_code)]
fn abc088_b() {
    input! {
    n_items: i32,
    arr: [i32; n_items],
    }

    let mut arr: Vec<i32> = arr;
    arr.sort();

    let res: i32 = arr.iter().rev()
        .enumerate()
        .map(|(i, &v)| {
            if (i % 2) == 0 {
                v
            } else {
                -v
            }
        })
        .sum();

    println!("{:?}", res)
}

#[allow(dead_code)]
fn abc085_b() {
    input! {
    n_: usize,
    arr: [[i32; 1]; n_],
    }

    let arr: Vec<Vec<i32>> = arr;
    let res = arr.iter()
//        .flatten()
        .flat_map(|a| a)
        .cloned()
        .collect::<HashSet<i32>>()
        .len();

    println!("{:?}", res)
}

#[allow(dead_code)]
fn abc085_c() {
    input! {
    args: [u32; 2],
    }

    let n: u32 = args[0];
    let total: u32 = args[1];

    let x_max = cmp::min(total / 10_000, n) + 1;

    for x in (0..x_max).rev() {
        if x * 10_000 > total { break; }
        let y_max = cmp::min(total / 5_000, n - x) + 1;
        for y in (0..y_max).rev() {
            let z = n - x - y;
            if x * 10_000 + y * 5_000 + z * 1_000 == total {
                println!("{} {} {}", x, y, z);
                return;
            }
        }
    }

    println!("-1 -1 -1")
}


#[allow(dead_code)]
fn arc065_a() {
    input! {
    arr: chars,
    }
    let mut arr: Vec<char> = arr;

    let dream: Vec<char> = "dream".chars().collect();
    let dreamer: Vec<char> = "dreamer".chars().collect();
    let erase: Vec<char> = "erase".chars().collect();
    let eraser: Vec<char> = "eraser".chars().collect();

    fn rel(n: i32) -> i32 {
        if n < 0 {
            0
        } else {
            n
        }
    }

    loop {
        let size = arr.len() as i32;
        if size == 0 {
            println!("YES");
            return;
        }

        let last_5 = rel(size - 5) as usize;
        let last_6 = rel(size - 6) as usize;
        let last_7 = rel(size - 7) as usize;

        let is_dream = arr[last_5..].to_vec() == dream;
        let is_dreamer = arr[last_7..].to_vec() == dreamer;
        let is_erase = arr[last_5..].to_vec() == erase;
        let is_eraser = arr[last_6..].to_vec() == eraser;

        if is_dream {
            arr.drain(last_5..);
        } else if is_dreamer {
            arr.drain(last_7..);
        } else if is_erase {
            arr.drain(last_5..);
        } else if is_eraser {
            arr.drain(last_6..);
        } else {
            println!("NO");
            return;
        }
    }
}

#[allow(dead_code)]
fn arc089_a() {
    input! {
    n_: usize,
    arr: [[u32; 3]; n_],
    }

    let arr: Vec<Vec<u32>> = arr;

    for a in arr {
        let t = a[0];
        let x = a[1];
        let y = a[2];

        // Check dist
        if x + y > t {
            println!("No");
            return;
        }

        if ((x + y) % 2) == 0 && (t % 2) == 0 {
            continue;
        }

        if ((x + y) % 2) == 1 && (t % 2) == 1 {
            continue;
        }

        println!("No");
        return;
    }

    println!("Yes");
}

#[allow(dead_code)]
fn abc075_b() {
    input! {
        h: i32,
        w: i32,
        board: [chars; h],
    }
    let board: Vec<Vec<char>> = board;

    for r in 0..h {
        for c in 0..w {
//            println!("{} {}", r, c);
            if board[r as usize][c as usize] == '#' {
                print!("#");
                continue;
            }

            let r_min = clamp(r - 1, 0, h) as usize;
            let r_max = clamp(r + 2, 0, h) as usize;
            let c_min = clamp(c - 1, 0, w) as usize;
            let c_max = clamp(c + 2, 0, w) as usize;

            let mut total = 0;

            for r_tmp in r_min..r_max {
                for c_tmp in c_min..c_max {
                    let val = board[r_tmp][c_tmp];
//                    println!("{} {} {} {} {}", val, r_min, r_max, c_min, c_max);
                    if val == '#' {
                        total += 1;
                    }
                }
            }
            print!("{}", total);
        }
        println!("");
    }
}

#[allow(dead_code)]
fn abc096_c() {
    input! {
        h: i32,
        w: i32,
        board: [chars; h],
    }
    let board: Vec<Vec<char>> = board;

    for r in 0..h {
        for c in 0..w {
            if board[r as usize][c as usize] == '.' {
                continue;
            }

            let r_min = clamp(r - 1, 0, h) as usize;
            let r_max = clamp(r + 2, 0, h) as usize;
            let c_min = clamp(c - 1, 0, w) as usize;
            let c_max = clamp(c + 2, 0, w) as usize;

            let mut has_neighbour = false;

            for r_tmp in r_min..r_max {
                for c_tmp in c_min..c_max {
                    if r_tmp == r as usize && c_tmp == c as usize {
                        continue;
                    }
                    if r_tmp != r as usize && c_tmp != c as usize {
                        continue;
                    }
                    let val = board[r_tmp][c_tmp];
                    if val == '#' {
                        has_neighbour = true;
                        break;
                    }
                }
            }

            if !has_neighbour {
                print!("No");
                return;
            }
        }
    }
    print!("Yes");
}

#[allow(dead_code)]
fn abc070_b() {
    input! {
        a0: u32,
        a1: u32,
        b0: u32,
        b1: u32,
    }
    let a: HashSet<u32> = (a0..a1).collect();
    let b: HashSet<u32> = (b0..b1).collect();
    let res = a.intersection(&b)
        .cloned()
        .collect::<HashSet<u32>>()
        .len();
    println!("{}", res);
}

#[allow(dead_code)]
fn abc055_b() {
    input! {
        n: u64,
    }
    let res = (1..n + 1)
        .fold(1, |acc, x| {
            (acc * x) % (1e+9 as u64 + 7)
        });
    println!("{}", res);
}

#[allow(dead_code)]
fn abc046_b() {
    input! {
        n: u32,
        m: u32,
    }
    let res = (1..n)
        .fold(m, |acc, _x| acc * (m - 1));
    println!("{}", res);
}

#[allow(dead_code)]
fn abc048_b() {
    input! {
        a: i64,
        b: i64,
        x: i64,
    }
    fn f(n: i64, x: i64) -> i64 {
        if n >= 0 {
            n / x + 1
        } else {
            0
        }
    }
    let res = f(b, x) - f(a - 1, x);
    println!("{}", res);
}

#[allow(dead_code)]
fn abc060_b() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    let res = (1..(b + 1))
        .map(|n| (a * n) % b == c)
        .any(|x| x);

    if res {
        println!("YES");
    } else {
        println!("NO");
    }
}

#[allow(dead_code)]
fn abc065_b() {
    input! {
        n: usize,
        arr: [usize; n],
    }
    let arr: Vec<usize> = arr;
    let mut cnt: usize = 0;
    let mut visited = vec![false; n];
    let mut pos: usize = 0;

    loop {
        if pos == 1 {
            println!("{}", cnt);
            return;
        }
        if visited[pos] {
            println!("{}", -1);
            return;
        }
        visited[pos] = true;
        cnt += 1;
        pos = arr[pos] - 1;
    }
}

#[allow(dead_code)]
fn arc090_a() {
    input! {
        n: usize,
        line0: [u32; n],
        line1: [u32; n],

    }
    let line0: Vec<u32> = line0;
    let line1: Vec<u32> = line1;

    let res = (0..n)
        .map(|i| {
            let sum0: u32 = line0[0..i + 1].iter().sum();
            let sum1: u32 = line1[i..n].iter().sum();
            sum0 + sum1
        })
        .max()
        .unwrap();

    println!("{}", res);
}

#[allow(dead_code)]
fn arc098_a() {
    input! {
        n: usize,
        arr: chars,
    }
    let arr: Vec<char> = arr;
    let mut w_sum_arr: Vec<usize> = vec![0; n + 1];
    let mut w_sum = 0;

    for i in 1..n + 1 {
        if arr[i - 1] == 'W' {
            w_sum += 1;
        }
        w_sum_arr[i] = w_sum;
    }

    let res = w_sum_arr[0..n].iter()
        .enumerate()
        .map(|(idx, &w_sum)| {
            let e_sum = n - w_sum_arr[n] - (idx + 1 - w_sum_arr[idx + 1]);
//            println!("{} {} {}", idx, w_sum, e_sum);
            w_sum + e_sum
        })
        .min()
        .unwrap();

    println!("{:?}", res);
}

#[allow(dead_code)]
fn arc098_a2() {
    input! {
        n: usize,
        arr: chars,
    }
    let arr: Vec<char> = arr;

    let w_sum_arr: Vec<usize> = (0..n)
        .scan(0, |state, i| {
            let x = *state;
            if arr[i] == 'W' {
                *state += 1;
            }
            Some(x)
        })
        .collect();

    let e_sum_arr: Vec<usize> = (0..n)
        .rev()
        .scan(0, |state, i| {
            let x = *state;
            if arr[i] == 'E' {
                *state += 1;
            }
            Some(x)
        })
        .collect();

    let res = w_sum_arr.iter()
        .zip(e_sum_arr.iter().rev())
        .map(|(w, e)| w + e)
        .min()
        .unwrap();

//    println!("{:?}", w_sum_arr);
//    println!("{:?}", e_sum_arr);
    println!("{:?}", res);
}

#[allow(dead_code)]
fn abc079_c() {
    input! {
        arr: chars,
    }
    let arr: Vec<char> = arr;
    let arr: Vec<i32> = arr.iter()
        .map(|&x| x.to_digit(10).unwrap() as i32)
        .collect();

    fn int_to_str(x: i32) -> String {
        if x >= 0 { format!("+{}", x) } else { format!("{}", x) }
    }

    for &i in [-1, 1].iter() {
        for &j in [-1, 1].iter() {
            for &k in [-1, 1].iter() {
                let a = arr[0];
                let b = arr[1] * i;
                let c = arr[2] * j;
                let d = arr[3] * k;
                if a + b + c + d == 7 {
                    let b = int_to_str(b);
                    let c = int_to_str(c);
                    let d = int_to_str(d);
                    println!("{}{}{}{}=7", a, b, c, d);
                    return;
                }
            }
        }
    }
}


fn main() {
    arc098_a2()
}

