use std::io;

fn exp(a: &i64, p: &i64) -> i64 {
    const MOD: i64 = 10_i64.pow(9) + 7;
    let mut base: i64 = *a;
    let mut exp: i64 = *p;
    let mut total: i64 = 1;
 
    while exp > 0 {
        if exp & 1 == 1 {
            total *= base;
            total %= MOD;
        }
        base *= base;
        base %= MOD;
        exp >>= 1;
    }
 
    total %= MOD;
 
    return total
}

fn g(n: &i64) -> i64 {
    let n: i64 = *n;
    let mut res: i64 = 0;
    const MOD: i64 = 10_i64.pow(9) + 7;
    
    if n < 2 {
        return 0
    }
    
    if n & 1 == 0 {
        let mut val: i64 = exp(&3, &(n/2 - 1));
        res += val;
        val *= 18;
        val %= MOD;
        res += val;
        res %= MOD;
    } else {
        let mut val: i64 = exp(&3, &((n - 1)/2));
        res += 2*val;
        res %= MOD;
        val *= 9;
        val %= MOD;
        res += val;
        res %= MOD;
    }
 
    res -= 11;
    
    while res < 0 {
        res += MOD;
    }
    
    res %= MOD;
    
    return res
}

fn s(n: &i64) -> i64 {
    const MOD: i64 = 10_i64.pow(9)  +7;
    let n: i64 = *n;
    let mut res: i64 = 0;
    
    if n & 1 == 0 {
        res += (g(&n) + g(&(n/2))) % MOD;
    } else {
        res += (g(&n) + g(&((n + 1)/2))) % MOD;
    }
    
    let inverse: i64 = 500000004;
    
    res *= inverse;
    res %= MOD;
    res += 4;
    res %= MOD;
    
    return res
}

fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let l: i64 = v[0].parse().expect("REASON");
    let r: i64 = v[1].parse().expect("REASON");

    const MOD: i64 = 10_i64.pow(9) + 7;
    
    if l == 1 {
      println!("{:?}", s(&r))  
    } else {
        let mut res: i64 = s(&r) - s(&(l - 1));
        while res < 0 {
            res += MOD;
        }
        
        res %= MOD;
        println!("{:?}", res)
    }
}
