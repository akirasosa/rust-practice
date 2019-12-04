fn main() {
//    for i in 0..1 << 10 {
//        println!("{}", i)
//    }
    let test1: Vec<Vec<usize>> = (0..1 << 3)
        .filter(|i: &usize| i.count_ones() == 2)
        .map(|i| {
            format!("{:018b}", i).chars()
                .rev()
                .map(|c| (c == '1') as usize)
                .collect()
        })
        .collect();

    println!("{:?}", test1);

    let test2: Vec<usize> = (0..1 << 3)
        .filter(|i: &usize| i.count_ones() == 2)
        .collect();
    println!("{:?}", test2);

    for t in test2 {
        println!("{:03b}", t);
        for i in 0..3 {
            println!("{}", t & (1 << i) > 0);
        }
    }
}