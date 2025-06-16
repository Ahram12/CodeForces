use std::io;
 
fn subsets(n: usize, m: usize) {
    let mut num_sets: usize = m;
    let mut total: usize = m;
    const MOD: usize = 10_usize.pow(9) + 7;
    
    for _ in 2..=n {
        num_sets *= m;
        num_sets %= MOD;
        total = (2*m - 1)*total;
        total %= MOD;
        total += num_sets;
        total %= MOD;
    }
    
    total += num_sets;
    total %= MOD;
    
    println!("{:?}", total)
}
 
fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let n: usize = v[0].parse().expect("REASON");
    let m: usize = v[1].parse().expect("REASON");
    
    subsets(n, m)
   
}
