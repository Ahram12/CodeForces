use std::io;
use std::cmp;
 
fn carousel(n: usize, cases: Vec<usize>) {
    let mut sieve_array: Vec<bool> = vec![true; n + 1];
    let mut primes: Vec<Vec<usize>> = vec![vec![]; n + 1];
    sieve_array[0] = false;
    sieve_array[1] = false;
 
    for i in 2..=n {
        if sieve_array[i] {
            primes[i].push(i);
            for j in (2*i..n + 1).step_by(i) {
                primes[j].push(i);
                sieve_array[j] = false;
            }
        }
    }
    
    let mut prev: i64 = 0;
    let mut s: Vec<i64> = vec![0; n + 1];
    
    const MOD: i64 = 10_i64.pow(9) + 7;
    
    for j in 2..=n {
        let m = primes[j].len();
        for i in 0..m {
            let residue = (j/primes[j][i]) % primes[j][i];
            if residue == 0 {
                prev -= 1;
            } else {
                prev -= ((primes[j][i] - (residue - 1)) % primes[j][i]) as i64;
                prev += ((primes[j][i] - residue)) as i64;
            }
            prev % MOD;
        }
        
        if j % 4 == 0 {
            let residue = (j/4) % 2;
            if residue == 0 {
                prev -= 2;
            } else {
                prev += 2;
            }
            prev %= MOD;
        }
        
        s[j] = s[j - 1] + prev;
        s[j] %= MOD;
    }
    
    let m = cases.len();
    for i in 0..m {
        println!("{:?}", s[cases[i]])
    }
 
}
 
fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let num_lines: usize = v[0].parse().expect("REASON");
    
    let mut cases: Vec<usize> = Vec::new();
    let mut max_value: usize = 0;
    
    for _ in 0..num_lines {
        let mut temp: String = String::new();
        
        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
        
        let w: Vec<&str> = temp.trim().split(' ').collect();
        let val = w[0].parse().expect("REASON");
        
        max_value = cmp::max(max_value, val);
        cases.push(w[0].parse().expect("REASON"));
    }
    
    carousel(max_value, cases)
   
}
