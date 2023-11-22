use std::time::Instant;

fn main(){
    let now = Instant::now();
    let primes =sieve(10000000);
    println!("primes: {:?}",  primes); 
    println!("number of primes found:{}", primes.len());
    println!("elapsed time: {:?}", now.elapsed());
    println!("last generated prime: {}", primes[primes.len()-1]);
}

fn sieve(n:u128) -> Vec<u128>{
    let mut primes = vec![true;(n+1) as usize];
    let mut p= 2;
    while p*p <=n {
        if primes[p as usize] {
            let mut index = p*p;
            while index <= n{
                primes[index as usize] =false;
                index += p;
            }
        }
        
        p +=1;
    }
    let mut result =vec![];
    for (i, p) in primes.iter().enumerate() {
       if *p {
           result.push(i as u128);
       }
    }
    result.remove(0);
    result.remove(0);
    result
}