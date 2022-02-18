//Problem 6
//Sum square difference

/*
The sum of the squares of the first ten natural numbers is 1*1 + 2*2 + ... + 10*10 = 385
The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)*(1 + 2 + ... + 10) = 55*55 = 3025,
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640
Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

fn main()
{
    let a: u32 = (1..=100).sum::<u32>().pow(2);
    let b: u32 = (1..=100).map(|x: u32| x.pow(2)).sum();
    let difference: u32 = a - b;
    println!("{} - {} = {}", a, b, difference);
}