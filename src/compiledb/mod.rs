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
    /// Command line : gen_compiledb.exe <buildlog> <compiler>
    /// <buidlog> : a file that is generatd while compiling
    /// <compiler> : cl2000 or gcc
    pub fn new(cwd: &str, input_args: &[String]) -> Result<LogPareser, ParseError> {
        if input_args.len() != 3 {
            return Err(ParseError::new(
                ParseErrorKind::InvalidParameter,
                format!("Wrong arguments length: {}", input_args.len()),
            ));
        }

        let cwd: String = String::from(cwd);
        let logfile = String::from(&input_args[1]);
        let compiler = String::from(&input_args[2]);

        Ok(LogPareser {
            cwd,
            logfile,
            compiler,
        })
    }

    pub fn generate(&self) -> Result<(), ParseError> {
        println!("The current directory is {}", self.cwd);
        println!("The input file is {}", &self.logfile);

        let compileinfo = CompileInfo::extract_buildinfo(&self.cwd, &self.compiler, &self.logfile);

        compileinfo.write_to_json();

        Ok(())
    }
}
