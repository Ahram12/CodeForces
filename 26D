use std::io;
 
fn tickets(n: i32, m: i32, k: i32) -> f64 {
    if n - m < -k {
        return 0.0_f64
    }
 
    let mut numer: i32 = m;
    let mut denom: i32 = n + k + 1;
    let mut prob: f64 = (numer as f64)/(denom as f64);
 
    for _ in 1..=k {
        numer -= 1;
        denom -= 1;
        if denom > 0 {
            prob *= (numer as f64)/(denom as f64);
        } else {
            break
        }
    }
 
    return 1.0 - prob
}
 
fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
 
    let n: i32 = v[0].parse().expect("REASON");
    let m: i32 = v[1].parse().expect("REASON");
    let k: i32 = v[2].parse().expect("REASON");
 
    println!("{:?}", tickets(n, m, k))
}
