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
    let n: usize = 7;
    let k: usize = 4;
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
    
    let f1: Field = exp(&alpha, &(n as i64 + 2));
    let f2: Field = exp(&beta, &(n as i64 + 2));
    let mut v: Field = Field{A: (n as i64) % MOD + 1, B: 0};
    
    let mut s: Vec<Field> = vec![subtract(&f1, &Field{A: 500000005, B: 500000004}); k + 1];
    let mut t: Vec<Field> = vec![subtract(&f2, &Field{A: 500000005, B: 500000003}); k + 1];
    
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
    
    
    for j in 1..=k {
        let mut temp1: Field = Field{A: 1, B: 0};
        let mut temp2: Field = Field{A: 1, B: 0};
        for i in 0..j {
            let a: Field = multiply(&Field{A: c[j][i], B: 0}, &s[i]);
            temp1 = add(&temp1, &a);
            let b: Field = multiply(&Field{A: c[j][i], B: 0}, &t[i]);
            temp2 = add(&temp2, &b);
        }
        temp1 = multiply(&temp1, &Field{A: 500000005, B: 500000004});
        temp2 = multiply(&temp2, &Field{A: 500000005, B: 500000003});
        
        temp1 = subtract(&multiply(&v, &f1), &temp1);
        temp2 = subtract(&multiply(&v, &f2), &temp2);
        
        s[j] = temp1.clone();
        t[j] = temp2.clone();
        
        v = multiply(&v, &Field{A: (n as i64) % MOD + 1, B: 0});
        
    }

    let mut total: Field = subtract(&multiply(&s[k], &alpha), &multiply(&t[k], &beta));
    total = multiply(&total, &gamma);
    
    println!("{:?}", total)
    
}
