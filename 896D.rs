use std::io;
use std::cmp;
 
fn inverse(val: &i64, prime: &i64, pow: &i64) -> i64 {
    let mut base = *val;
    let modulo: i64 = prime.pow(*pow as u32);
 
    if base % modulo == 0 {
        return 0
    } else {
        let mut res: i64 = 1;
        let mut power: i64 = (modulo/(*prime))*(*prime - 1) - 1;
 
        while power > 0 {
            if power & 1 == 1 {
                res = (res * base) % modulo;
            }
            base = (base * base) % modulo;
            power >>= 1;
        }
 
        return res
    }
}
 
fn main() {
    let mut s: String = String::new();
     
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
 
    let v: Vec<&str> = s.trim().split(' ').collect();
 
    let n: usize = v[0].parse().expect("REASON");
    let p: usize = v[1].parse().expect("REASON");
    let l: usize = v[2].parse().expect("REASON");
    let r: usize = v[3].parse().expect("REASON");
    
    if p == 1 {
        println!("{:?}", 0)
    } else {
    
        let mut temp: usize = p;
        let sqrt = (p as f64).sqrt() as usize;
        let mut sieve_array: Vec<bool> = vec![true; sqrt + 1];
        let mut primes: Vec<Vec<i64>> = Vec::new();
        
        sieve_array[0] = false;
        sieve_array[1] = false;
        
        for i in 2..=sqrt {
            if sieve_array[i] {
                if temp % i == 0 {
                    let mut pow: i64 = 0;
                    while temp % i == 0 {
                        temp /= i;
                        pow += 1;
                    }
                    primes.push(vec![i as i64, pow]);
                }
                for j in (2*i..sqrt + 1).step_by(i) {
                    sieve_array[j] = false;
                }
            }
        }
        
        if temp != 1 {
            primes.push(vec![temp as i64, 1]);
        }
        
        let m = primes.len();
        let mut factorial_res: Vec<Vec<i64>> = vec![vec![1; n + 1]; m];
        let mut inverse_res: Vec<Vec<i64>> = vec![vec![1; n + 1]; m];
        let mut factorial_pow: Vec<Vec<i64>> = vec![vec![0; n + 1]; m];
        
        for j in 0..m {
            let val = primes[j][0].pow(primes[j][1] as u32);
            for i in 1..=n {
                if i as i64 % primes[j][0] == 0 {
                    let mut curr: i64 = i as i64;
                    let mut pow: i64 = 0;
                    while curr % primes[j][0] == 0 {
                        curr /= primes[j][0];
                        pow += 1;
                    }
                    factorial_pow[j][i] = factorial_pow[j][i - 1] + pow;
                    curr %= val;
                    factorial_res[j][i] = curr*factorial_res[j][i - 1];
                    factorial_res[j][i] %= val;
                } else {
                    factorial_pow[j][i] = factorial_pow[j][i - 1];
                    factorial_res[j][i] = (i as i64 % val)*factorial_res[j][i - 1];
                    factorial_res[j][i] %= val;
                }
                inverse_res[j][i] = inverse(&factorial_res[j][i], &primes[j][0], &primes[j][1]);
            } 
        }
        
        let mut sums: Vec<i64> = vec![0; m];
        
        for i in 0..m {
            if l == 0 {
                sums[i] = 1;
            }
            if l <= 1 && 1 <= r {
                let val = primes[i][0].pow(primes[i][1] as u32);
                sums[i] += n as i64 % val;
                sums[i] %= val;
            }
        }
        
        for i in 0..m {
            let val = primes[i][0].pow(primes[i][1] as u32);
            for j in (0..n-1).rev() {
                let mut first: i64 = 0;
                if factorial_pow[i][n] < primes[i][1] + factorial_pow[i][j] + factorial_pow[i][n - j] {
                    first = 1;
                    first *= factorial_res[i][n] * ((inverse_res[i][j] * inverse_res[i][n - j]) % val);
                    first %= val;
                    first *= primes[i][0].pow((factorial_pow[i][n] - factorial_pow[i][n - j] - factorial_pow[i][j]) as u32);
                    first %= val;
                }
                let t = cmp::min(r, n - j);
                let mut second: i64 = 0;
                let mut third: i64 = 0;
                if t + 2 >= n - j {
                    if t == n - j && t >= l {
                        second += 1;
                    }
                    if l + 2 <= n - j {
                        let mut min_k = l;
                        if ((n - j) - l) % 2 != 0 {
                            min_k = l + 1;
                        } 
                        let s = ((n - j) - min_k)/2;
                        if factorial_pow[i][n - j] < primes[i][1] + factorial_pow[i][s] + factorial_pow[i][n - j - s] {
                            third = 1;
                            third *= factorial_res[i][n - j] * ((inverse_res[i][s] * inverse_res[i][n - j - s]) % val);
                            third %= val;
                            third *= primes[i][0].pow((factorial_pow[i][n - j] - factorial_pow[i][s] - factorial_pow[i][n - j - s]) as u32);
                            third %= val;
                        }
                        third -= 1;
                    }
                    second += third;
                    second %= val;
                } else {
                    if t > l {
                        let mut max_k: usize = t;
                        let mut min_k: usize = l;
                        if ((n - j) - t) % 2 != 0 {
                            max_k = t - 1;
                        }
                        
                        if ((n - j) - l) % 2 != 0 {
                            min_k = l + 1;
                        }
                        
                        if max_k >= min_k {
                            let s1 = ((n - j) - min_k)/2;
                            let s2 = (((n - j) - max_k)/2) - 1;
                            
                            if factorial_pow[i][n - j] < primes[i][1] + factorial_pow[i][s1] + factorial_pow[i][n - j - s1] {
                                let mut temp: i64 = 1;
                                temp *= factorial_res[i][n - j] * ((inverse_res[i][s1] * inverse_res[i][n - j - s1]) % val);
                                temp %= val;
                                temp *= primes[i][0].pow((factorial_pow[i][n - j] - factorial_pow[i][s1] - factorial_pow[i][n - j - s1]) as u32);
                                temp %= val;
                                third += temp;
                                third %= val;
                            }
                            
                            if factorial_pow[i][n - j] < primes[i][1] + factorial_pow[i][s2] + factorial_pow[i][n - j - s2] {
                                let mut temp: i64 = 1;
                                temp *= factorial_res[i][n - j] * ((inverse_res[i][s2] * inverse_res[i][n - j - s2]) % val);
                                temp %= val;
                                temp *= primes[i][0].pow((factorial_pow[i][n - j] - factorial_pow[i][s2] - factorial_pow[i][n - j - s2]) as u32);
                                temp %= val;
                                third -= temp;
                                third %= val;
                            }
                        }
                    } else if t == l && ((n - j) - t) % 2 == 0 {
                        let max_k = t;
                        let min_k = t;
                        let s1 = (n - j - min_k)/2;
                        let s2 = ((n - j - max_k)/2) - 1;
                        
                         
                        if factorial_pow[i][n - j] < primes[i][1] + factorial_pow[i][s1] + factorial_pow[i][n - j - s1] {
                            let mut temp: i64 = 1;
                            temp *= factorial_res[i][n - j] * ((inverse_res[i][s1] * inverse_res[i][n - j - s1]) % val);
                            temp %= val;
                            temp *= primes[i][0].pow((factorial_pow[i][n - j] - factorial_pow[i][s1] - factorial_pow[i][n - j - s1]) as u32);
                            temp %= val;
                            third += temp;
                            third %= val;
                        }
                            
                        if factorial_pow[i][n - j] < primes[i][1] + factorial_pow[i][s2] + factorial_pow[i][n - j - s2] {
                            let mut temp: i64 = 1;
                            temp *= factorial_res[i][n - j] * ((inverse_res[i][s2] * inverse_res[i][n - j - s2]) % val);
                            temp %= val;
                            temp *= primes[i][0].pow((factorial_pow[i][n - j] - factorial_pow[i][s2] - factorial_pow[i][n - j - s2]) as u32);
                            temp %= val;
                            third -= temp;
                            third %= val;
                        }
                    } else {
                    }
                    second += third;
                }
                first *= second;
                first %= val;
                sums[i] += first;
                sums[i] %= val;
            }
        }
        
        let mut curr_res: i64 = sums[0];
        let mut curr_mod: i64 = primes[0][0].pow(primes[0][1] as u32);
        
        for i in 1..m {
            let new_mod = primes[i][0].pow(primes[i][1] as u32);
            let mut update = inverse(&curr_mod, &primes[i][0], &primes[i][1]);
            update = (sums[i] - curr_res)*update;
            update %= new_mod;
            curr_res += curr_mod*update;
            curr_mod *= new_mod;
            curr_res %= curr_mod;
        }
        
        while curr_res < 0 {
            curr_res += curr_mod;
        }
        
        curr_res %= curr_mod;
        
        println!("{:?}", curr_res)
    }
    
}
