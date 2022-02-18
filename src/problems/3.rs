//Problem 3
//Largest prime factor

/*
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600 851 475 143 ?
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

fn factors(number: &u64) -> Vec<u64>
{
    let mut primes = Primes::new();
    let mut current = primes.next().unwrap();
    
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


fn main()
{
    let n = 600_851_475_143;
    
    let factors = factors(&n);

    println!("Factors = {:?} (Max = {})", factors, factors.iter().max().unwrap());
}