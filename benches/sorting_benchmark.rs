use criterion::{
    black_box,
    criterion_group, 
    criterion_main, 
    Criterion};


pub fn find_min_combi(n: i32, lock_1: &str, lock_2: &str) -> usize {
    let mut sum = 0; // O(1)
    let digits1: Vec<u8> = lock_1.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(); // O(n)
    let digits2: Vec<u8> = lock_2.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(); // O(n)

    for i in 0..n as usize { // O(n)
        let d1 = digits1[i]; // O(1)
        let d2 = digits2[i]; // O(1)
        let mut diff = (d1 as i32 - d2 as i32).abs() as usize; // O(1)
        if diff > 5 { // O(1)
            diff = 10 - diff; // O(1)
        }
        sum += diff; // O(1)
    }
    sum
}

pub fn sorting_benchmark(c: &mut Criterion) {
    c.bench_function("Min Combi", |b| b.iter(|| find_min_combi((100), ("0408508214408693401443322277782286096051410454577137791407339268821842297598596429618918019769521388"), 
    ("5747993954668983025628583105482716255958863659698708329400634030065794924167009144046862895933022878"))));
}

criterion_group!(benches, sorting_benchmark);
criterion_main!(benches);