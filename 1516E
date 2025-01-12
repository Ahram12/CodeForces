use std::io;
use std::cmp;
 
fn inverse(val: &i64) -> i64 {
    let mut base = *val;
    const MOD: i64 = 10_i64.pow(9) + 7;
 
    if base % MOD == 0 {
        return 0
    } else {
        let mut res: i64 = 1;
        let mut power: i64 = 10_i64.pow(9) + 5;
 
        while power > 0 {
            if power & 1 == 1 {
                res = (res * base) % MOD;
            }
            base = (base * base) % MOD;
            power >>= 1;
        }
 
        return res
    }
}
 
fn permutations(n: usize, k: usize) {
    let num_cases: usize = k;
    let k: usize = cmp::min(k, n);
    const MOD: i64 = 10_i64.pow(9) + 7;
    let mut f: Vec<i64> = vec![1; k + 2];
    let mut g: Vec<i64> = vec![1; k + 2];
    let mut h: Vec<i64> = vec![0; k + 2];
    
    for i in 0..=k {
        f[i + 1] = (i as i64 + 1)*f[i];
        f[i + 1] %= MOD;
        g[i + 1] = inverse(&f[i + 1]);
        h[i + 1] = inverse(&(i as i64 + 1));
    }
    
    let mut c: Vec<i64> = vec![0; 2*k + 1];
    c[0] = 1;
    for i in 0..cmp::min(n, 2*k) {
        c[i + 1] = c[i] * (n - i) as i64;
        c[i + 1] %= MOD;
    }
    
    let mut s: Vec<Vec<i64>> = vec![vec![0; k + 1]; k + 1];
    let mut t: Vec<i64> = vec![0; k + 1];
    t[0] = 1;
    
    for m in 1..= k {
        for j in m..=cmp::min(k, n - m) {
            if m == 1 {
                s[m][j - m] = h[j - m + 2];
            } else {
                for i in 0..=j - m {
                    s[m][j - m] += (s[m - 1][j - m - i] * h[i + 2]) % MOD;
                    s[m][j - m] %= MOD;
                }
            }
            t[j] += (s[m][j - m] * ((c[m + j] * g[m]) % MOD)) % MOD;
            t[j] %= MOD;
        }
    }
    
    for i in 2..=k {
        t[i] += t[i - 2];
        t[i] %= MOD;
    }
    
    for i in 1..=num_cases {
        if i < num_cases && i < n + 1 {
            print!("{ } { }", t[i - 1], ' ')
        } else if i < num_cases && i >= n + 1 {
            print!("{ } { }", t[k], ' ')
        } else if i == num_cases && i < n + 1 {
            print!("{ }", t[i - 1])
        } else {
            print!("{ }", t[k])
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
    let k: usize = v[1].parse().expect("REASON");
    
    permutations(n, k)
   
}
