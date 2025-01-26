use std::io;
 
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
 
fn stars(n: i64) ->  i64 {
    const MOD: i64 = 998244353_i64;
    let mut f: Vec<i64> = vec![1; n as usize + 1];
    let mut c: Vec<i64> = vec![1; n as usize + 1];
   
    for i in 0..n as usize {
        f[i + 1] = (i as i64 + 1)*f[i];
        f[i + 1] %= MOD;
    }
   
    for i in 0..=n as usize/2 {
        c[i] = (((f[n as usize]*inverse(&f[i], &MOD)) % MOD) * inverse(&f[n as usize - i], &MOD)) % MOD;
    }
   
    for i in n as usize/2 + 1..=n as usize {
        c[i] = c[n as usize - i];
    }
   
    let mut s1: i64 = 0;
    let mut pow: i64 = (-1_i64).pow(n as u32);
    let mut base: i64 = 1;
   
    for i in (1..=n as usize).rev() {
        pow *= -1;
        let a: i64 = pow * c[i];
        let c: i64 = exp(&(base - 1), &n, &MOD);
        let d: i64 = exp(&base, &n, &MOD);
        s1 += (a*(c - d)) % MOD;
        base *= 3;
        base %= MOD;
    }
   
    s1 *= 3;
    s1 %= MOD;
   
    let mut s2: i64 = (exp(&base, &n, &MOD) - exp(&(base - 3), &n, &MOD)) % MOD;
    s2 *= 2;
    s2 %= MOD;
   
    let mut s = s2 + s1;
    while s < 0 {
        s += MOD;
    }
   
    s %= MOD;
   
   
    return s
}
 
fn main() {
    let mut s: String = String::new();
 
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
 
    let v: Vec<&str> = s.trim().split(' ').collect();
 
    let n: i64 = v[0].parse().expect("REASON");
    
    println!("{:?}", stars(n))
}
