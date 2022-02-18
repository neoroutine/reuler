pub mod problem;
use crate::problem::Problem;

use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 { println!("Usage: reuler <problem_number>"); return}
    println!("Opening problem ({})", &args[1]);

    let problem = Problem::new(&args[1]).run();

    match problem
    {
        Ok(o) => { println!("Problem ({}) \"{}\" :\nOutput:\n{}", o.number(), o.name(), o.output()); o.clean();},
        Err(e) => println!("Error : {:?}", e),
    }

}
