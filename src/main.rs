use std::env;

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

fn p4(input : usize) -> usize {
    todo!()
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
                _ => println!("Problem Unsolved Currently!"),
            }
        }
        Err(_) => println!("Please provide a valid problem number!"),
    }
}