fn main() {
    let n: usize = 100;
    let m: usize = 100;
    let mut f: Vec<[f64; 2]> = vec![[1.0,0.0]; m];
    let mut expectation: Vec<f64> = vec![0.0; m];
    
    for _ in 0..n {
        for i in 2..=m {
            if i == 2 {
                f[i - 1][0] = 2.0*f[i - 2][0];
                f[i - 1][1] = -3.0 - expectation[i - 2];
            } else {
                f[i - 1][0] = -f[i - 3][0] + 3.0*f[i - 2][0];
                f[i - 1][1] = -f[i - 3][1] + 3.0*f[i - 2][1] - expectation[i - 2] - 4.0;
            }
        }
        
        let temp = (3.0 + f[m - 2][1] + expectation[m - 1] - 2.0*f[m - 1][1])/(2.0*f[m - 1][0] - f[m -2][0]);
        
        for i in 0..m {
            if i <= (m - 1)/2 {
                expectation[i] = temp*f[i][0] + f[i][1];
            } else {
                expectation[i] = expectation[m - 1 - i];
            }
        }
    }
    
    println!("{:?}", expectation)
    
}
