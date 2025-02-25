use criterion::{
    black_box,
    criterion_group, 
    criterion_main, 
    Criterion};


pub fn segui(n: i32, lock_1: &str, lock_2: &str) -> usize {
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
    c.bench_function("segui 1", |b| b.iter(|| segui(black_box(100), black_box("2550695163887675125926664564272695912558013187143196255732112165066273558452835913913807890350419737"), black_box("7575763888851660554175962374665952046037782551747468939605907411172359061491734970454812196267863210"))));
}

criterion_group!(benches, sorting_benchmark);
criterion_main!(benches);