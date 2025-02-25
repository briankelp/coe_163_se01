pub fn segui(n: i32, lock_1: &str, lock_2: &str) {
    let mut sum = 0;
    let digits1: Vec<u8> = lock_1.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    let digits2: Vec<u8> = lock_2.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

    for i in 0..n as usize {
        let d1 = digits1[i];
        let d2 = digits2[i];
        let mut diff = (d1 as i32 - d2 as i32).abs() as usize;
        if diff > 5 {
            diff = 10 - diff;
        }
        sum += diff;
    }
    println!("{}", sum);
}