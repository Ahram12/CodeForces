//Work in Progress
fn power(matrix: &Vec<Vec<i64>>, pow: &i64) -> Vec<Vec<i64>> {
    let mut base: Vec<Vec<i64>> = matrix.clone();
    let mut exp: i64 = *pow;
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
            total = multiplication(&total, &base).clone();
        }
        base = multiplication(&base, &base).clone();
        exp >>= 1;
    }
    
    return total

}



fn multiplication(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let m = a.len();
    let p = b[0].len();
    const MOD: i64 = 998_244_352_i64;
    
    let mut res: Vec<Vec<i64>> = vec![vec![0; p]; m];
    
    for k in 0..m {
        for j in 0..p {
            let mut sum: i64 = 0;
            for i in 0..m {
                sum += (a[k][i]*b[i][j]) % MOD;
                sum %= MOD;
            }
            res[k][j] = sum;
        }
    }
    
    return res
}

fn main() {
    let k: usize = 5;
    let n: usize = 7;
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; k]; k];
    
    for i in 0..k {
        for j in 0..k {
            if i < k - 1 {
                if i + 1 == j {
                    matrix[i][j] = 1;
                }
            }
        }
    }
    matrix.pop();
    matrix.push(vec![6, 5, 1, 7, 4]);
    
    matrix = power(&matrix, &((n - k) as i64)).clone();
    
    
    
    println!("{:?}", matrix)    
}
