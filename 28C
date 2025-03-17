use std::io;

fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let n: usize = v[0].parse().expect("REASON");
    let m: usize = v[1].parse().expect("REASON");
    
    let mut temp_string: String = String::new();

    io::stdin()
        .read_line(&mut temp_string)
        .expect("Failed to read line");

    let temp_v: Vec<&str> = temp_string.trim().split(' ').collect();
    let mut arr: Vec<usize> = Vec::new();
    
    for i in 0..m {
        let temp: usize = temp_v[i].parse().expect("REASON");
        arr.push(temp);
    }
    
    let mut p: Vec<Vec<Vec<f64>>> = vec![vec![vec![0.0; n + 1]; n + 1]; m];
    let mut cdf: Vec<Vec<Vec<f64>>> = vec![vec![vec![1.0; n + 1]; n + 1]; m];
    let mut c: Vec<Vec<f64>> = vec![vec![0.0; n + 1]; n + 1];
    
    for i in 0..=n {
        for j in 0..=i {
            if j == 0 || j == i {
                c[i][j] = 1.0;
            } else {
                c[i][j] = c[i - 1][j] + c[i - 1][j - 1];
            }
        }
    }
    
    for j in 0..=n {
        for k in 1..=n {
            let val: usize = ((j as f64)/(arr[0] as f64)).ceil() as usize;
            if val == k {
                p[0][j][k] = 1.0;
            }
        }
        
        for k in (0..n).rev() {
            cdf[0][j][k] = cdf[0][j][k + 1] - p[0][j][k + 1];
        }
    }
    
    for i in 1..m {
        let frac: f64 = 1.0/(i as f64 + 1.0);
        for j in 1..=n {
            for k in 1..=n {
                for s in 0..=j {
                    let val: usize = ((s as f64)/(arr[i] as f64)).ceil() as usize;
                    let prob: f64 = c[j][s]*frac.powf(s as f64)*(1.0 - frac).powf((j - s) as f64);
                    if val < k {
                        p[i][j][k] += prob*p[i - 1][j - s][k];
                    } else if val == k {
                        p[i][j][k] += prob*cdf[i - 1][j - s][k];
                    }
                }
            }
            
            for k in (0..n).rev() {
                cdf[i][j][k] = cdf[i][j][k + 1] - p[i][j][k + 1];
            }
        }
    }
    
    let mut exp: f64 = 0.0;
    for k in 1..=n {
        exp += (k as f64)*p[m - 1][n][k];
    }
    
    println!("{:?}", exp)
}
