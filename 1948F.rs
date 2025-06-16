use std::io;
use std::cmp;
 
fn exp(a: &usize, p: &usize, q: &usize) -> usize {
    let modulus: usize = *q;
    let mut base: usize = *a;
    let mut exp: usize = *p;
    let mut total: usize = 1;
 
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
 
fn inverse(val: &usize, prime: &usize) -> usize {
    let base = *val;
    let modulus: usize = *prime;
 
    if base % modulus == 0 {
        return 0
    } else {
        return exp(&base, &(modulus - 2), &modulus)
    }
}
 
fn rare_coins(gold: Vec<usize>, silver: Vec<usize>, queries: Vec<Vec<usize>>) {
    let n = gold.len();
    let mut gold_prefix: Vec<usize> = vec![0; n + 1];
    let mut silver_prefix: Vec<usize> = vec![0; n + 1];
    const MOD: usize = 998244353_usize;
    
    for i in 0..n {
        gold_prefix[i + 1] = gold_prefix[i] + gold[i];
        silver_prefix[i + 1] = silver_prefix[i] + silver[i];
    }
    
    let mut f: Vec<usize> = vec![1; silver_prefix[n] + 1];
    let mut g: Vec<usize> = vec![1; silver_prefix[n] + 1];
    
    for i in 0..silver_prefix[n] {
        f[i + 1] = (i + 1) * f[i];
        f[i + 1] %= MOD;
        g[i + 1] = inverse(&f[i + 1], &MOD);
    }
    
    let mut cdf: Vec<usize> = vec![0; silver_prefix[n] as usize + 2];
    
    cdf[silver_prefix[n] as usize + 1] = 0;
    let mut p = inverse(&2, &MOD);
    p = exp(&p, &silver_prefix[n], &MOD);
    
    for i in (0..=silver_prefix[n]).rev() {
        cdf[i] = (((f[silver_prefix[n]]*g[i] % MOD)*g[silver_prefix[n] - i]) % MOD * p) % MOD;
        cdf[i] += cdf[i + 1];
        cdf[i] %= MOD;
        
    }
    
    let m = queries.len();
    
    for i in 0..m {
        let t: i64 = gold_prefix[n] as i64 - (2*(gold_prefix[queries[i][1]] - gold_prefix[queries[i][0] - 1])) as i64;
        let y: i64 = silver_prefix[n] as i64 - (silver_prefix[queries[i][1]] - silver_prefix[queries[i][0] - 1]) as i64;
        if i < m - 1 {
            print!("{ } { }", cdf[cmp::min(silver_prefix[n] + 1, cmp::max(t + y + 1, 0) as usize)], ' ')
        } else {
            print!("{ }", cdf[cmp::min(silver_prefix[n] + 1, cmp::max(t + y + 1, 0) as usize)])
        }
        
    }
    
    
}
 
 
fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let n: usize = v[0].parse().expect("REASON");
    let q: usize = v[1].parse().expect("REASON");
    
    let mut gold: Vec<usize> = Vec::new();
    let mut silver: Vec<usize> = Vec::new();
    let mut queries: Vec<Vec<usize>> = Vec::new();
    
    for j in 0..q + 2 {
        let mut m: String = String::new();
        
        io::stdin()
        .read_line(&mut m)
        .expect("Failed to read line");
        
        let w: Vec<&str> = m.trim().split(' ').collect();
        if j == 0 {
            for i in 0.. n {
                gold.push(w[i].parse().expect("REASON"));
            }
        } else if j == 1 {
            for i in 0..n {
                silver.push(w[i].parse().expect("REASON"));
            }
        } else {
            queries.push(vec![w[0].parse().expect("REASON"), w[1].parse().expect("REASON")]);
        }
    }
    
    
    rare_coins(gold, silver, queries)
}
