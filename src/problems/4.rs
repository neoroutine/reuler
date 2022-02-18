//Problem 4
//Largest palindrome product

/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.
*/

//TODO convert primes to u32


fn reversed(number: &i32) -> i32
{
    let radix = 10;
    let mut n = *number;
    let mut reversed = 0;

    while n != 0 
    {
        reversed = reversed * radix + n % radix;
        n /= radix;
    }

    reversed
}

fn main()
{
    let mut largest = 0;
    let mut a = 0;
    let mut b = 0;

    'outer: for i in (0..999).rev()
    {
        for j in (0..999).rev()
        {
            if i*j == reversed(&(i*j)) { largest = i*j; a = i; b = j; break 'outer;}
        }
    }

    println!("Largest palindrome is {} from {} * {}", largest, a, b);
}