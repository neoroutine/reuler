//Problem 5
//Smallest multiple

/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

//TODO convert primes to u32

#[derive(Debug)]
struct Primes
{
    computed: Vec<u64>,
    number: u64,
}

impl Primes
{
    fn new() -> Primes
    {
        Primes { computed: Vec::new(), number: 2}
    }
}

impl Iterator for Primes
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.computed.len() == 0
        {
            self.computed.push(self.number);
            return Some(self.number)
        }

        let mut found = false;

        let mut current = self.number;


        while !found
        {
            let mut prime = true;
            current += 1;

            for p in &self.computed
            {
                if current % p == 0 { prime = false; break;}
            }

            if prime { found = true;}
        }

        self.computed.push(current);
        self.number = current;

        Some(self.number)
    }
}

fn factors(number: &u64) -> Vec<u64>
{
    let mut primes = Primes::new();
    let mut current: u64 = primes.next().unwrap();
    
    let mut m = *number;
    let mut factors: Vec<u64> = Vec::new();

    while (m - current) > 0
    {
        if m % current == 0
        {
            m /= current;
            factors.push(current);
        }
        else
        {
            current = primes.next().unwrap();
        }
    }

    factors.push(current);
    
    factors
}

fn lcm(numbers: &Vec<u64>) -> u64
{
    let mut all: Vec<Vec<u64>> = Vec::new();

    for number in numbers
    {
        all.push(factors(number));
    }

    let mut coefs: Vec<u32> = Vec::new();
    let mut primes = Primes::new();

    let mut current: u64 = 0;
    let max = numbers.iter().max().unwrap();
    while current < *max
    {
        current = primes.next().unwrap();

        let mut local_coefs: Vec<u32> = Vec::new();

        for fl in &all
        { 
            local_coefs.push(fl.into_iter().filter(|x| **x == current).count() as u32);
        }

        coefs.push(*local_coefs.iter().max().unwrap())
    }

    let mut lcm = 1;

    let mut primes = Primes::new();

    for c in &coefs
    {
        current = primes.next().unwrap();

        lcm *= current.pow(*c as u32);
    }

    lcm
}

fn main()
{
    let numbers: Vec<u64> = (2..20).collect();

    println!("LCM = {}", lcm(&numbers));
}