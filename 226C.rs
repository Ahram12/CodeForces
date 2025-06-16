use std::io;
use std::cmp;

fn power(matrix: &Vec<Vec<i64>>, pow: &i64, modulus: &i64) -> Vec<Vec<i64>> {
    let mut base: Vec<Vec<i64>> = matrix.clone();
    let mut exp: i64 = *pow;
    let modulus: i64 = *modulus;
    let mut total: Vec<Vec<i64>> = vec![vec![0; matrix.len()]; matrix.len()];
    
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if i == j {
                total[i][j] = 1;
            }
        }
    }
    
    while exp > 0 {
        if exp & 1 == 1 {
            total = multiplication(&total, &base, &modulus).clone();
        }
        base = multiplication(&base, &base, &modulus).clone();
        exp >>= 1;
    }
    
    return total

}


fn multiplication(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>, modulus: &i64) -> Vec<Vec<i64>> {
    let m = a.len();
    let p = b[0].len();
    
    let mut res: Vec<Vec<i64>> = vec![vec![0; p]; m];
    
    for k in 0..m {
        for j in 0..p {
            let mut sum: i64 = 0;
            for i in 0..m {
                sum += (a[k][i]*b[i][j]) % *modulus;
                sum %= *modulus;
            }
            res[k][j] = sum;
        }
    }
    
    return res
}

fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let m: i64 = v[0].parse().expect("REASON");
    let l: i64 = v[1].parse().expect("REASON");
    let r: i64 = v[2].parse().expect("REASON");
    let k: i64 = v[3].parse().expect("REASON");

    let mut d: i64 = 0;
    
    let sqrt: i64 = (r as f64).sqrt() as i64;
    for i in 1..=sqrt {
        if r/i - (l-1)/i >= k {
            d = cmp::max(d, i);
        }
        
        let temp = r/i;
        if r/temp - (l - 1)/temp >= k {
            d = cmp::max(d, r/i);
        }
    }
    
    let mut matrix: Vec<Vec<i64>> = vec![vec![1, 1], vec![1, 0]];
    matrix = power(&matrix, &(d - 1), &m).clone();
    
    println!("{:?}", (matrix[1][0] + matrix[1][1]) % m)
}
