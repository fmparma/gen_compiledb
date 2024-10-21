mod parser;

use std::fs;
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
        
        let contents: String = fs::read_to_string(&self.logfile).unwrap();

        let result: u8 = Self::search_gen_compdb(self, &contents);
        match result {
            1 => Err(ParseError::new(ParseErrorKind::InnerErr,
                                    format!("generate error"))),
            _ => Ok(())
        }
    }

    #[allow(unused)]
    fn search_gen_compdb(&self, contents: &str) -> u8 {
        // 
        let mut compinfo = CompileInfo::new();
        let mut saveonce: u8 = 0;

        compinfo.add_directory(&self.cwd);

        for line in contents.lines() {
            // search line include "cl2000"
            //println!("Compiler is {}",&self.compiler);
            if line.contains(&self.compiler) {
                //
                let splitedconts = line.split_whitespace();

                if saveonce == 0 {
                    //
                    compinfo.add_compileinfo(splitedconts);
                    saveonce = 1;
                } else {
                    //
                    compinfo.add_srcfile(splitedconts.into_iter().last().unwrap());
                }
            }
        }

        if saveonce == 0 {
            // Doesn't find any compile source.
            return 1;
        } else {
            compinfo.write_to_json();
            return 0;
        }
    }
}

