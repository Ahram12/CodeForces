use std::io;

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


fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
 
    let k: usize = v[0].parse().expect("REASON");
    let l: i64 = v[1].parse().expect("REASON");
    let r: i64 = v[2].parse().expect("REASON");
    const MOD: i64 = 10_i64.pow(9) + 7;
    
    #[derive(Debug, Clone)]
    #[allow(non_snake_case)]
    struct Field {
        A: i64,
        B: i64,
    }
    
    fn add(a: &Field, b: &Field) -> Field {
        let mut x = (a.A + b.A) % MOD;
        let mut y = (a.B + b.B) % MOD;
        
        while x < 0 {
            x += MOD;
        }
        
        while y < 0 {
            y += MOD;
        }
        
        return Field{A: x % MOD, B: y % MOD}
    }
    
    fn subtract(a: &Field, b: &Field) -> Field {
        let mut x =  (a.A - b.A) % MOD;
        let mut y =  (a.B - b.B) % MOD;
        
        while x < 0 {
            x += MOD;
        }
        
        while y < 0 {
            y += MOD;
        }
        
        return Field{A: x % MOD, B: y % MOD}
    }
    
    fn multiply(a: &Field, b: &Field) -> Field {
        let mut x = (a.A*b.A + (5*((a.B*b.B) % MOD) % MOD)) % MOD;
        let mut y = (((a.A*b.B) % MOD) + ((a.B*b.A) % MOD)) % MOD;
        
        while x < 0 {
            x += MOD;
        }
        
        while y < 0 {
            y += MOD;
        }
        
        return Field{A: x % MOD, B: y % MOD}
        
    }
    
    fn inversion(a: &Field) -> Field {
        let mut denom = (((a.A).pow(2) % MOD) - ((5*(a.B.pow(2) % MOD)) % MOD)) % MOD;
        while denom < 0 {
            denom += MOD;
        }
        denom %= MOD;
        let inverse = inverse(&denom);
        let x = Field{A: a.A, B: -a.B};
        let y = Field{A: inverse, B: 0};
        return multiply(&x, &y)
    }
    
    fn exp(a: &Field, r: &i64) -> Field {
        let mut base = a.clone();
 
        if (base.A % MOD == 0) && (base.B % MOD == 0) {
            return Field{A: 0, B: 0}
        } else {
            let mut res: Field = Field{A: 1, B: 0};
            let mut power = *r;
 
            while power > 0 {
                if power & 1 == 1 {
                    res = multiply(&res, &base);
                    res.A %= MOD;
                    res.B %= MOD;
                }
                base = multiply(&base, &base);
                base.A %= MOD;
                base.B %= MOD;
                power >>= 1;
            }
 
            while res.A < 0 {
                res.A += MOD;
            }
            
            while res.B < 0 {
                res.B += MOD;
            }
            
            res.A %= MOD;
            res.B %= MOD;
            
            return res
        }
    }
    
    let alpha: Field = Field{A: 500000004, B: 500000004};
    let beta: Field = Field{A: 500000004, B: 500000003};
    let gamma: Field = Field{A: 0, B: 400000003};
    
    let mut alpha_pow: Vec<Field> = vec![Field{A: 1, B: 0}; k + 1];
    let mut beta_pow: Vec<Field> = vec![Field{A: 1, B: 0}; k + 1];
    let mut gamma_pow: Vec<Field> = vec![Field{A: 1, B: 0}; k + 1];
    
    for i in 1..=k {
        alpha_pow[i] = multiply(&alpha_pow[i - 1], &alpha);
        beta_pow[i] = multiply(&beta_pow[i - 1], &beta);
        gamma_pow[i] = multiply(&gamma_pow[i - 1], &gamma);
    }
    
    let mut c: Vec<Vec<i64>> = vec![vec![0; k + 1]; k + 1];
    for j in 0..=k {
        for i in 0..=j {
            if i == 0 || i == j {
                c[j][i] = 1;
            } else {
                c[j][i] = c[j - 1][i] + c[j - 1][i - 1];
                c[j][i] %= MOD;
            }
        }
    }
    
    let mut s: Vec<i64> = vec![0; k + 1];
    for j in 1..=k {
        s[j] = 1;
        for i in (1..j).rev() {
            s[i] = s[i - 1] - (j as i64 - 1)*s[i];
            s[i] %= MOD;
        }
    }
    
    let mut upper_sum: Field = Field{A: 0, B: 0};
    let mut lower_sum: Field = Field{A: 0, B: 0};
    for j in 0..=k {
        let mut temp_upper_sum: Field = Field{A: 0, B: 0};
        let mut temp_lower_sum: Field = Field{A: 0, B: 0};
        for m in 0..=j {
            let first = multiply(&Field{A: c[j][m], B: 0}, &Field{A: (-1_i64).pow((j - m) as u32), B: 0});
            let product = multiply(&alpha_pow[m], &beta_pow[j - m]);
            if product.A == 1 && product.B == 0 {
                temp_upper_sum = add(&temp_upper_sum, &multiply(&first, &Field{A: (r + 3) % MOD, B: 0}));
                temp_lower_sum = add(&temp_lower_sum, &multiply(&first, &Field{A: (l + 2) % MOD, B: 0}));
            } else {
                let numer_upper = subtract(&exp(&product, &(r + 3)), &Field{A: 1, B: 0});
                let numer_lower = subtract(&exp(&product, &(l + 2)), &Field{A: 1, B: 0});
                let denom = inversion(&subtract(&product, &Field{A: 1, B: 0}));
                let second_upper = multiply(&numer_upper, &denom);
                let second_lower = multiply(&numer_lower, &denom);
                let final_upper = multiply(&first, &second_upper);
                let final_lower = multiply(&first, &second_lower);
                temp_upper_sum = add(&temp_upper_sum, &final_upper);
                temp_lower_sum = add(&temp_lower_sum, &final_lower);
            }
        }
        temp_upper_sum = multiply(&temp_upper_sum, &gamma_pow[j]);
        temp_upper_sum = multiply(&temp_upper_sum, &Field{A: s[j], B: 0});
        temp_lower_sum = multiply(&temp_lower_sum, &gamma_pow[j]);
        temp_lower_sum = multiply(&temp_lower_sum, &Field{A: s[j], B: 0});
        upper_sum = add(&upper_sum, &temp_upper_sum);
        lower_sum = add(&lower_sum, &temp_lower_sum);
    }
    
    let mut factorial: i64 = 1;
    for i in 1..=k {
        factorial *= i as i64;
        factorial %= MOD;
    }
    
    factorial = inverse(&factorial);
    let mut total = upper_sum.A - lower_sum.A;
    while total < 0 {
        total += MOD;
    }
    
    total %= MOD;
    total *= factorial;
    total %= MOD;
    
    println!("{:?}", total)
}
