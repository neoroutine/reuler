use std::process::Command;
use std::path::PathBuf;
use std::fs::{File, self};
use std::io::{BufRead, BufReader};

pub struct Problem
{
    number: String,
    name: String,
    src_path: PathBuf,
    exe_path: PathBuf, //TODO:Make that cross platform
    compiled: bool,
    ran: bool,
    output: String
}
#[derive(Debug)]
pub enum ProblemError
{
    ProblemNotFound,
    NotAProblem,
    CompilationError,
    NotCompiled,
    NotRan,
    RunError,
    CleanError,
}



fn name_from_number(num: &String) -> Result<String, ProblemError>
{
    match File::open(format!("src/problems/{}.rs", num))
    {
        Ok(f) => match BufReader::new(f).lines().nth(1)
        {
            Some(l) => match l
            {
                Ok(s) => if let Ok(c) = String::from_utf8(s.as_bytes()[2..].to_vec()) { Ok(c) } else { Ok(String::from("")) },
                Err(_) => Ok(String::from("")),
            },
            None => Err(ProblemError::NotAProblem),
            
        }
        Err(_) => Err(ProblemError::ProblemNotFound)
    }
    
}

impl Problem
{
    pub fn new(num: &String) -> Self
    {
        Problem
        {
            number: String::from(num),
            name: if let Ok(n) = name_from_number(num) { n } else { String::from("") } ,
            src_path: PathBuf::from(format!("src/problems/{}.rs", num)),
            exe_path: PathBuf::from(format!("src/problems/{}.exe", num)),
            compiled: false,
            ran: false,
            output: String::from(""),
        }
    }

    pub fn number(&self) -> String
    {
        String::from(&self.number)
    }

    pub fn name(&self) -> String
    {
        String::from(&self.name)
    }

    pub fn output(&self) -> String
    {
        String::from(&self.output)
    }

    fn compile(mut self) -> Result<Self, ProblemError> 
    {
        if self.compiled
        {
            Ok(self)
        }
        else
        {
            match Command::new("rustc").args(&[&self.src_path, &"-o".into(), &self.exe_path]).output()
            {
                Ok(o) => 
                {
                    let err = String::from_utf8_lossy(&o.stderr).to_string();
                    if err.len() != 0 { println!("\n{}\n",err);}
                    self.compiled = true; 
                    Ok(self)
                },

                Err(_) => Err(ProblemError::CompilationError),
            }
        }
    }

    fn execute(mut self) -> Result<Self, ProblemError>
    {
        if !self.compiled
        {
            Err(ProblemError::NotCompiled)
        }
        else
        {
            match Command::new(&self.exe_path).output()
            {
                Ok(o) =>
                {
                    self.ran = true;
                    self.output = String::from_utf8_lossy(&o.stdout).to_string();
                    Ok(self)
                },
                Err(e) => {println!("Execution error:\n{}", e.to_string());Err(ProblemError::RunError)},
            }
        }
    }

    pub fn run(self) -> Result<Self, ProblemError>
    {
        if self.compiled && self.ran
        {
            Ok(self)
        }
        else
        {
            match self.compile()
            {
                Ok(o) => match o.execute()
                {
                    Ok(o) => Ok(o),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            }
        }
        
    }

    pub fn clean(self) -> Self
    {
        fs::remove_file(&self.exe_path).ok();
        fs::remove_file(PathBuf::from(format!("src/problems/{}.pdb", &self.number()))).ok();

        self    
    }
}