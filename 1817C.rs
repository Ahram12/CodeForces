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
 
        return res % MOD
    }
}

fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let d: usize = v[0].parse().expect("REASON");

    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();

    for i in 0..2 {
        let mut temp_string: String = String::new();

        io::stdin()
            .read_line(&mut temp_string)
            .expect("Failed to read line");

        let temp_v: Vec<&str> = temp_string.trim().split(' ').collect();

        if i == 0 {
            for j in 0..=d {
                a.push(temp_v[j].parse().expect("REASON"));
            }
        } else {
            for j in 0..=d {
                b.push(temp_v[j].parse().expect("REASON"));
            }
        }
    }

    const MOD: i64 = 10_i64.pow(9) + 7;
    let mut val: i64 = 1;
    let mut inverse_val: i64 = 1;
    let mut inverses: Vec<i64> = vec![1; d + 1];

    
    for i in 1..=d {
        val *= i as i64;
        val %= MOD;
        inverse_val *= inverse(&(i as i64));
        inverse_val %= MOD;
        inverses[i] = inverse_val;
    }
    
    let mut c: Vec<i64> = vec![1; d/2 + 1];
    for i in 0..=d/2 {
        c[i] = (val*((inverses[i]*inverses[d - i]) % MOD)) % MOD;
    }
    
    let mut numer: i64 = 0;
    let mut pow: i64 = (-1_i64).pow(d as u32 + 1);
    let mut denom: i64 = 0;
    let v: i64 = (((d + 1)*d)/2) as i64 % MOD;
    
    for i in 0..=d {
        numer += (((((c[cmp::min(i, d - i)])*(v - i as i64)) % MOD) * (b[i] - a[i])) % MOD) * pow;
        numer %= MOD;
        pow *= -1;
        denom += ((c[cmp::min(i, d - i)] * a[i]) % MOD)*pow;
        denom %= MOD;
        
    }
    
    while denom < 0 {
        denom += MOD;
    }
    
    denom *= d as i64;
    denom %= MOD;
    
    let temp = inverse(&denom);
    numer *= temp;
    
    while numer < 0 {
        numer += MOD;
    }
    
    numer %= MOD;

    println!("{:?}", numer)
    
}
