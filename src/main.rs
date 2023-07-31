use std::{ env, collections };

mod helpers;

macro_rules! error {
    ( $num:expr ) => {
        println!("Please provide a valid input for Problem {}!",$num)
    }
}

fn p1(upper : usize) -> usize {
    (1..upper).fold(0, |acc, x| if ( x.rem_euclid(3) == 0 ) || ( x.rem_euclid(5) == 0 ) { acc + x } else { acc } )
}

fn p2(max : usize) -> usize {
    
    use crate::helpers::fib;

    let mut fibs = Vec::new();

    let mut n = 2;

    loop {
        match fib(n) {
            num if num <= max => {
                fibs.push( num );
                n += 1;
            }
            _ => break,
        }
    }

    fibs.iter().filter(|x| x.rem_euclid(2) == 0 ).sum()

}

fn p3(num : usize) -> usize {
    
    use crate::helpers::factorize;

    match factorize(num).into_iter().map( |(p,_)| p ).max() {
        Some(largest) => largest,
        None => unreachable!(),
    }
}

fn p4(length : usize) -> usize {

    use crate::helpers::{ nums_by_length, is_palindrome };


    let mut maximum = 0;

    for n in nums_by_length(10,length) {
        for m in nums_by_length(10,length) {
            let mult = n*m;
            let palindrome = is_palindrome(mult);
            
            if palindrome && mult >= maximum {
                maximum = mult;
            }
        }
    }
    return maximum
}

fn p5(upper: usize) -> usize {

    use collections::HashSet;
    use crate::helpers::{ unfactorize, factorize };


    // Flattened list of all prime factor, prime power pairs that occur in any factorization
    // of a number in the range 1..upper+1
    let factorizations = (1..upper+1).map(|x| factorize(x) ).flatten().collect::<Vec<_>>();

    // Set of primes that occur in factorizations
    let primes = factorizations.iter().map(|(p,_)| *p ).collect::<HashSet<usize>>();

    let mut result = Vec::new();

    for p in primes {
        match factorizations.iter().filter(|(f,_)| p == *f ).max_by(|(p1,t1),(p2,t2)| t1.cmp(t2) ) {
            Some(t) => result.push((p,t.1)),
            None => unreachable!(),
        }
    }

    unfactorize(result)

}

fn p6(upper: usize) -> usize {
    (1..upper+1).sum::<usize>().pow(2) - (1..upper+1).map( |x: usize| x.pow(2) ).sum::<usize>()
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let problem = &args[1];
    let argument = &args[2];



    match problem.parse::<usize>() {
        Ok(num) => {
            match num {
                1 => {
                    match argument.parse::<usize>() {
                        Ok(arg) => println!("{}",p1(arg)),
                        Err(_) => error!(num),
                    }
                }
                2 => {
                    match argument.parse::<usize>() {
                        Ok(arg) => println!("{}",p2(arg)),
                        Err(_) => error!(num),
                    }
                }
                3 => {
                    match argument.parse::<usize>() {
                        Ok(arg) => println!("{}",p3(arg)),
                        Err(_) => error!(num),
                    }
                }
                4 => {
                    match argument.parse::<usize>() {
                        Ok(arg) => println!("{}",p4(arg)),
                        Err(_) => error!(num),
                    }
                }
                5 => {
                    match argument.parse::<usize>() {
                        Ok(arg) => println!("{}",p5(arg)),
                        Err(_) => error!(num),
                    }
                }
                6 => {
                    match argument.parse::<usize>() {
                        Ok(arg) => println!("{}",p6(arg)),
                        Err(_) => error!(num),
                    }
                }
                _ => println!("Problem Unsolved Currently!"),
            }
        }
        Err(_) => println!("Please provide a valid problem number!"),
    }
}
