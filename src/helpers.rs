pub fn fib( n : usize ) -> usize {
    if n == 0 || n == 1 { 1 } else { fib( n - 1 ) + fib( n - 2 ) }
}

fn factor_bound( n : usize ) -> usize {
    (n as f64).sqrt() as usize +1
}


pub fn is_prime( n : usize ) -> bool {
    if n == 1 { false } else {
        for i in 2..factor_bound(n) {
            if n.rem_euclid(i) == 0 { return false }
        }
        true
    }
    
}

pub fn factorize( n : usize ) -> Vec<(usize,usize)> {
    let mut m = n;
    let mut result = Vec::new();
    let mut i = 0;
    let mut p = 1;
    while m != 1 {
        while !is_prime(p) {
            p += 1;
        }
        if m % p == 0 {
            result.push((p,1 as usize));
            m = m / p;
            while m % p == 0 {
                result[i].1 += 1;
                m = m / p;
            }
        }
        else {
            result.push((p,0));
            p += 1;
        }
        i += 1;
    }
    result
}