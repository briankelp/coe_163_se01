use criterion::{
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
    c.bench_function("Min Combi", |b| b.iter(||  find_min_combi(50,
        "15292304008095093550618514054452538304493692938923", 
        "23986022971705733498711898030829527051135426763781")));
}

criterion_group!(benches, sorting_benchmark);
criterion_main!(benches);