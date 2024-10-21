use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::SplitWhitespace;

pub struct CompileInfo {
    directory: String,
    srcfiles: Vec<String>,
    compiler: String,
    include_path: Vec<String>,
    defines: Vec<String>,
}

impl CompileInfo {
    // Extract compilation information
    pub fn new(cwd: &str, compilername: &str, logfile: &str) -> CompileInfo {
        let directory = cwd.to_string();
        let mut srcfiles = Vec::new();
        let mut compiler = String::new();
        let mut include_path = Vec::new();
        let mut defines = Vec::new();

        let contents = fs::read_to_string(logfile).unwrap();
        let mut saveonce: u8 = 0;

        for line in contents.lines() {
            // if include the compiler name
            if line.contains(compilername) {
                let splited_txt = line.split_whitespace();

                if saveonce == 0 {
                    //
                    for info in splited_txt.into_iter() {
                        if info.contains(compilername) {
                            compiler.push_str(info);
                        } else if info.contains("--include_path=") {
                            include_path.push(info.get(15..).unwrap().to_string());
                        } else if info.contains("--define=") {
                            defines.push(info.get(9..).unwrap().to_string());
                        } else if info.contains(".c")
                            || info.contains(".asm")
                            || info.contains(".C")
                        {
                            srcfiles.push(info.to_string());
                        }
                    }
                    saveonce = 1;
                } else {
                    let info = splited_txt.into_iter().last().unwrap();
                    if info.contains(".c") || info.contains(".asm") || info.contains(".C") {
                        srcfiles.push(info.to_string());
                    }
                }
            }
        }

        CompileInfo {
            directory,
            srcfiles,
            compiler,
            include_path,
            defines,
        }
    }

    #[allow(unused)]
    pub fn add_directory(&mut self, directory: &str) {
        self.directory.push_str(directory);
    }

    #[allow(unused)]
    pub fn add_srcfile(&mut self, file: &str) {
        // Only *.c, *.C or *.asm file to save
        if file.contains(".c") || file.contains(".asm") || file.contains(".C") {
            self.srcfiles.push(file.to_string());
        }
    }

    #[allow(unused)]
    pub fn add_compileinfo(&mut self, infos: SplitWhitespace) {
        //
        for cont in infos.into_iter() {
            //
            if cont.contains("cl2000") {
                // compiler
                self.compiler.push_str(cont);
            } else if cont.contains("--include_path=") {
                // include path
                self.include_path.push(cont.get(15..).unwrap().to_string());
            } else if cont.contains("--define=") {
                // define parameter
                self.defines.push(cont.get(9..).unwrap().to_string());
            } else if cont.contains(".c") || cont.contains(".asm") || cont.contains(".C") {
                // source file
                self.srcfiles.push(cont.to_string());
            }
        }
    }

    /// Write compilation database to 'compile_commnads.json'
    pub fn write_to_json(&self) -> u8 {
        //
        //let mut json_file: String = String::new();
        //json_file.push_str(&self.directory);
        //json_file.push_str("\\compile_commands.json");
        let json_file: String = String::from("compile_commands.json");

        if let Ok(_file) = File::create(json_file) {
            let mut file = OpenOptions::new()
                .append(true)
                .open("compile_commands.json")
                .expect("cannot open file");

            file.write_all("[\n".as_bytes()).expect("write failed");

            // Directory
            let mut directory: String = String::from("  \"directory\": \"");
            let tmp: String = self.directory.replace("\\", "\\\\");
            directory.push_str(&tmp);
            directory.push_str("\",\n");

            // compiler
            let mut compiler: String = String::from("      ");
            let tmp: String = self.compiler.replace("/", "\\\\");
            compiler.push_str(&tmp);
            compiler.push_str(",\n");

            // include
            let mut include_str: String = String::new();
            for include_path in &self.include_path {
                //
                include_str.push_str("      \"-I\",\n");
                include_str.push_str("      ");
                //include_str.push_str(include_path);
                let tmp: String = include_path.replace("/", "\\\\");
                include_str.push_str(&tmp);
                include_str.push_str(",\n");
            }

            // defines
            let mut define_str: String = String::new();
            for definename in &self.defines {
                //
                define_str.push_str("      \"-D\",\n");
                define_str.push_str("      \"");
                define_str.push_str(definename);
                define_str.push_str("\",\n");
            }

            // Generates contents for each source file
            for srcfile in &self.srcfiles {
                file.write_all(" {\n".as_bytes()).expect("write failed");

                // directory
                file.write_all(directory.as_bytes()).expect("write failed!");

                // source file
                let mut line: String = String::from("  \"file\": ");
                let mut tmp: String = srcfile.replace("../", ".\\\\");
                tmp = tmp.replace("/", "\\\\");
                line.push_str(&tmp);
                line.push_str(",\n");
                file.write_all(line.as_bytes()).expect("write failed!");

                // arguments start
                file.write_all("  \"arguments\": [\n".as_bytes())
                    .expect("write failed");
                // compiler
                file.write_all(compiler.as_bytes()).expect("write failed!");
                //file.write_all("     \"gcc\",\n".as_bytes()).expect("write failed");
                // include
                file.write_all(include_str.as_bytes())
                    .expect("write failed!");
                // defines
                file.write_all(define_str.as_bytes())
                    .expect("write failed!");
                // argument end
                file.write_all("   ]\n".as_bytes()).expect("write failed");

                file.write_all(" },\n".as_bytes()).expect("write failed");
            }

            // json file end
            file.write_all("]".as_bytes()).expect("write failed");
            // print all source files
            //let mut i: u32 = 1;
            //for srcfile in &self.srcfiles {
            //println!("{} : {}", i, srcfile);
            //i += 1;
            //}
            return 0;
        } else {
            return 1;
        }
    }
}
