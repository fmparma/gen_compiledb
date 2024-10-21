mod parser;

use crate::errhandle::{ParseError, ParseErrorKind};
use parser::CompileInfo;

// struct defination
pub struct LogPareser {
    cwd: String,
    logfile: String,
    compiler: String,
}

impl LogPareser {
    //
    pub fn new(cwd: &str, input_args: &[String]) -> Result<LogPareser, ParseError> {
        if input_args.len() != 2 {
            return Err(ParseError::new(ParseErrorKind::InvalidParameter, 
                                      format!("Wrong arguments length: {}", input_args.len())));
        }

        let cwd: String = String::from(cwd);
        //let mut logfile = cwd.clone();
        //logfile.push('\\');
        //logfile.push_str(&input_args[1]);
        let logfile = String::from(&input_args[1]);

        Ok(
            LogPareser {
            cwd,
            logfile,
            compiler: String::from("cl2000"),
        })
    }

    pub fn generate(&self) -> Result<(), ParseError> {
        println!("The current directory is {}", self.cwd);
        println!("The input file is {}", &self.logfile);
        
        let compileinfo = CompileInfo::new(&self.cwd, &self.compiler, & self.logfile);

        compileinfo.write_to_json();

        Ok(())
    }

}
