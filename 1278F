use std::io;
 
fn inverse(val: &i64) -> i64 {
    let mut base = *val;
    const MOD: i64 = 998244353_i64;
 
    if base % MOD == 0 {
        return 0
    } else {
        let mut res: i64 = 1;
        let mut power: i64 = 998244351_i64;
 
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
fn cards(n: i32, m: i32, k: i32) -> i64 {
    const MOD: i64 = 998244353_i64;
    let mut c: Vec<Vec<i64>> = vec![vec![0; k as usize + 1]; k as usize + 1];
 
    for j in 0..=k as usize {
        for i in 0..=j {
            if i ==0 || i == j {
                c[j][i] = 1;
            } else {
                c[j][i] = c[j - 1][i] + c[j - 1][i - 1];
                c[j][i] %= MOD;
            }
        }
    }
 
    let mut s: Vec<i64> = vec![0; k as usize + 1];
    s[0] = 1;
    let p: i64 = inverse(&(m as i64));
 
    for j in 1..=k as usize {
        for i in 0..j {
            s[j] += ((((n as i64 + 1)*c[j - 1][i]) % MOD) - c[j][i])*s[i];
            s[j] %= MOD;
        }
        s[j] *= p;
        s[j] %= MOD;
    }
 
   while s[k as usize] < 0 {
       s[k as usize] += MOD;
   }
 
   return s[k as usize] % MOD
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
    
    println!("{:?}", cards(n, m, k))
}
