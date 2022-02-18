//Problem 7
//10001st prime

/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10 001st prime number?
*/

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

fn main()
{
    let primes = Primes::new();
    println!("10_001st prime = {}", primes.skip(10_000).next().unwrap());
}