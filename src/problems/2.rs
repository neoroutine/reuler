//Problem 2
//Even Fibonacci numbers

/*
Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
*/

fn main()
{
    let mut sum: u32     = 0;
    let mut last: u32    = 1;
    let mut current: u32 = 2;

    while current < 4_000_000
    {
        if current % 2 == 0 { sum += current;}


        let tmp = current;
        current += last;
        last = tmp;
    }


    println!("Sum = {}", sum);
}