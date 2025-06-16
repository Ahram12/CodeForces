use std::io;
use std::cmp;

fn exp(a: &i64, p: &i64, q: &i64) -> i64 {
    let modulus: i64 = *q;
    let mut base: i64 = *a;
    let mut exp: i64 = *p;
    let mut total: i64 = 1;
 
    while exp > 0 {
        if exp & 1 == 1 {
            total *= base;
            total %= modulus;
        }
        base *= base;
        base %= modulus;
        exp >>= 1;
    }
 
    total %= modulus;
 
    return total
}
 
fn inverse(val: &i64, prime: &i64) -> i64 {
    let base = *val;
    let modulus: i64 = *prime;
 
    if base % modulus == 0 {
        return 0
    } else {
        return exp(&base, &(modulus - 2), &modulus)
    }
}

fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let n: usize = v[0].parse().expect("REASON");
    let k: usize = v[1].parse().expect("REASON");
    const MOD: i64 = 10_i64.pow(9) + 7;
    
    let mut temp_string: String = String::new();

    io::stdin()
        .read_line(&mut temp_string)
        .expect("Failed to read line");

    let temp_v: Vec<&str> = temp_string.trim().split(' ').collect();
    let mut sum: i64 = 0;
    for i in 0..n {
        let temp: i64 = temp_v[i].parse().expect("REASON");
        sum += temp;
        sum %= MOD;
    }
    
    let mut prod: i64 = 1;
    let mut inverses: Vec<i64> = vec![1; k];
    let mut exponents: Vec<i64> = vec![1; k];
    
    for i in 1..k {
        prod *= inverse(&(i as i64), &MOD);
        prod %= MOD;
        inverses[i] = prod;
        exponents[i] = exp(&(i as i64 + 1), &(n as i64 - 2), &MOD);
    }
    
    let mut c: Vec<i64> = vec![1; k];
    for i in 0..=(k - 1)/2 {
        c[i] = (inverses[i]*inverses[k - 1 - i]) % MOD;
    }
    
    let mut total: i64 = 0;
    let mut pow: i64 = if k % 2 == 0 {-1} else {1};
    
    for i in 0..k {
        let mut temp: i64 = 1;
        temp *= pow*((((c[cmp::min(i, k - 1 - i)]*exponents[i]) % MOD) * (n + i) as i64) % MOD);
        total += temp;
        total %= MOD;
        pow *= -1;
    }
    
    total *= sum;
    total %= MOD;
    
    while total < 0 {
        total += MOD;
    }
    
    
    println!("{:?}", total)
    
}
