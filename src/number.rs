use std::collections::HashSet;

use primal;
use rayon::prelude::*;

#[derive(Debug)]
enum Unit {
    One,
    Neg,
    Im,
    NIm,
    NegIm,
    OneIm,
    Zero,
}

impl Sign {
    fn get( num: isize ) -> Self {
        if num > 0 {
            Sign::Positive,
        }
        else if num < 0 {
            Sign::Negative,
        }
        else {
            Sign::Zero,
        }
    }

    fn non_positive( &self ) -> bool {
        match self {
            Sign::Positive  => false,
            Sign::Negative  => true,
            Sign::Zero      => true,
        }
    }

    fn non_negative( &self ) -> bool {
        match self {
            Sign::Positive  => true,
            Sign::Negative  => false,
            Sign::Zero      => true,
        }
    }

    fn non_zero( &self ) -> bool {
        match self {
            Sign::Positive  => true,
            Sign::Negative  => true,
            Sign::Zero      => false,
        }
    }

}

#[derive(Debug)]
struct Representation<'a> {
    base: u8,
    presentation: &'a [u8]
}

impl Representation<'_> {
    fn get<'a>( sign: &'a Sign, factorization: &'a Factorization<'a> ) -> Self {
        let presentation = vec![]
    }
}

#[derive(Debug)]
struct Factorization<'a> {
    prime_powers: Box<[Number<'a>]>,
    divisors: HashSet<&'a Number<'a>>,
    unitary: HashSet<&'a Number<'a>>,
}

#[derive(Debug)]
pub struct Number<'a> {
    sign: Sign,
    representation: Representation<'a>,
    factorization : Factorization<'a>,
}


// Creates a Number from a usize
impl From<usize> for Number<'_> {
    fn from( num : usize ) -> Number {
        let sign = Sign::Positive if num > 0 else 
        
        Natural {
            n,
            factorization : factorize( n ),
        }
    }
}
/*
// Creates a Natural from a u64
impl From<u64> for Natural {
    fn from( num : u64 ) -> Natural {
        Natural {
            n,
            factorization : factorize( n ),
        }
    }
}

// Creates a Natural from a u32 Factorization
impl From<Vec<u32>> for Natural {
    fn from( factorization: Vec<u32> ) -> Natural {
        Natural {
            n : factorization
                .iter()
                .enumerate()
                .fold( 1, |res, (i,a)| res * primal::StreamingSieve::nth_prime(i+1).pow((*a).try_into().unwrap()) ),
            factorization,
        }
    }
}
// Creates a Natural from a usize Factorization
impl From<Vec<usize>> for Natural {
    fn from( factorization: Vec<usize> ) -> Natural {
        Natural {
            n : factorization
                .iter()
                .enumerate()
                .fold( 1, |res, (i,a)| res * primal::StreamingSieve::nth_prime(i+1).pow((*a).try_into().unwrap()) ),
            factorization,
        }
    }
}
// Creates a Natural from a u64 Factorization
impl From<Vec<u64>> for Natural {
    fn from( factorization: Vec<u64> ) -> Natural {
        Natural {
            n : factorization
                .iter()
                .enumerate()
                .fold( 1, |res, (i,a)| res * primal::StreamingSieve::nth_prime(i+1).pow((*a).try_into().unwrap()) ),
            factorization,
        }
    }
}

impl Natural {

    fn add( self, other : Natural ) -> Natural {
        Self::from( self.n + other.n )
    }

    fn multiply( self, other : Natural ) -> Natural {
        Self::from( self.factorization
            .par_iter()
            .zip( other.factorization )
            .map( |(a,b)| a + b )
            .collect::<Vec<_>>() )
    }

}

 */
fn factorize( n : usize ) -> Box<[usize]> {
    // Highly Parallel
    // First, check if n is in memoized primes
    // If not, then check primality (cheaply?)
    // If not prime, then get upperbound primefactor, P, using n/logn ( because of Prime Number theorem )
    // Identify prime power upperbound, p_K, for every prime smaller than primefactor upperbound ( in parallel ) O( n^0.525 )
    // For every prime p < P:
    //  For every primepower k < p_K:
    //      p_k += 1 if n mod p**k == 0
    // ^ This entire outer loop can be parallelized trivially
    // The interior loop can be parallelized using atomic increments
    // The result being that complexity drops to roughly:
    // 1. Check membership of n
    // 2. Check primality of n
    // 3. Calculate upperbound P
    // .. All steps now happen for every prime p < P in parallel
    // 4. Find upperbound p_K such that p**p_K <= n
    // .. All steps now happen for every primepower 0 < k <= p_K in parallel
    // 5. p_k += 1 if n mod p**k == 0
}

fn factorize( n : u64 ) -> Box<[u64]> {
    let mut m = n;
    let mut result : Vec<u64> = vec![];
    let mut i = 0;
    while m != 1 {
        let p = primal::StreamingSieve::nth_prime(i+1) as u64;
        if m % p == 0 {
            result.push(1);
            m = m / p;
            while m % p == 0 {
                result[i] += 1;
                m = m / p;
            }
        }
        else {
            result.push(0);
        }
        i += 1;
    }
    result.into_boxed_slice()
}
