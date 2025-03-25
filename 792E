use std::io;
use std::cmp;

fn helper(m: &i64, n: &i64) -> bool {
    let m: i64 = *m;
    let n: i64 = *n;
    let upper = ((n as f64)/(m as f64)).floor() as i64;
    let lower = ((n as f64)/(m as f64 + 1.0)).ceil() as i64; 
    
    return upper >= lower
}

fn main() {
    let mut n: String = String::new();
 
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
 
    let v: Vec<&str> = n.trim().split(' ').collect();
    let num_cases: usize = v[0].parse().expect("REASON");

    let mut arr: Vec<i64> = Vec::new();
    let mut n: i64 = i64::MAX;

    let mut temp: String = String::new();
        
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let w: Vec<&str> = temp.trim().split(' ').collect();

    for i in 0..num_cases {
        let val: i64 = w[i].parse().expect("REASON");
        n = cmp::min(n, val);
        arr.push(val);
    }

    let sqrt: i64 = (n as f64).sqrt() as i64;
    let mut res: Vec<i64> = Vec::new();
    
    for i in 1..=sqrt {
        if helper(&i, &n) {
            res.push(i);
        }
        
        let temp: i64 = n/i;
        if helper(&temp, &n) {
           res.push(temp);
           res.push(temp - 1);
        }
    }
    
    res.sort_unstable();
    res.reverse();
    res.dedup();
    
    let mut d: i64 = 1;
    
    for case in &res {
        let mut flag: bool = true;
        for num in &arr {
            if !helper(case, num) {
                flag = false;
                break
            }
        }
        if flag {
            d = *case;
            break
        }
    }
    
    let mut total: i64 = 0;
    for num in arr {
        let res: i64 = num % d;
        let j: i64 = (num - res*(d + 1))/(d * (d + 1));
        let x = res + j*d;
        let y = (num - (d + 1)*x)/d;
        total += x + y;
    }
    
 
    
    println!("{:?}", total)
}
