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
    ///
    pub fn new(cwd: &str) -> CompileInfo {
        CompileInfo {
            directory: String::from(cwd),
            srcfiles: Vec::new(),
            compiler: String::new(),
            include_path: Vec::new(),
            defines: Vec::new(),
        }
    }

    /// Extract compilation information
    pub fn extract_buildinfo(cwd: &str, compilername: &str, logfile: &str) -> CompileInfo {
        let mut buildinfo = Self::new(cwd);

        let contents = fs::read_to_string(logfile).unwrap();
        let mut saveonce: u8 = 0;

        for line in contents.lines() {
            // if include the compiler name
            if line.contains(compilername) {
                let splited_txt = line.split_whitespace();

                if saveonce == 0 {
                    // All compiler information same, save once
                    buildinfo.add_compileinfo(compilername, splited_txt);
                    saveonce = 1;
                } else {
                    let info = splited_txt.into_iter().last().unwrap();
                    buildinfo.add_srcfile(info);
                }
            }
        }

        return buildinfo;
    }

    #[allow(unused)]
    fn add_directory(&mut self, directory: &str) {
        self.directory.push_str(directory);
    }

    #[allow(unused)]
    fn add_srcfile(&mut self, file: &str) {
        // Only *.c, *.C or *.asm file to save
        // 'cl2000' includes '*.cla' file
        if file.contains(".c")
            || file.contains(".asm")
            || file.contains(".C")
            || file.contains(".cla")
        {
            self.srcfiles.push(file.to_string());
        }
    }

    #[allow(unused)]
    fn add_compileinfo(&mut self, compilername: &str, infos: SplitWhitespace) {
        // Setting the filter pattern depend on compiler "cl2000" or "gcc"
        let mut include_pattern = "-I\"";
        let mut define_pattern = "-D";
        let mut include_get_idx = 2;
        let mut define_get_idx = 2;
        if compilername.contains("cl2000") {
            include_pattern = "--include_path=";
            define_pattern = "--define=";
            include_get_idx = 15;
            define_get_idx = 9;
        }

        for cont in infos.into_iter() {
            //
            if cont.contains(compilername) {
                // compiler
                self.compiler.push_str(cont);
            } else if cont.contains(include_pattern) {
                // include path
                self.include_path
                    .push(cont.get(include_get_idx..).unwrap().to_string());
            } else if cont.contains(define_pattern) {
                // define parameter
                self.defines
                    .push(cont.get(define_get_idx..).unwrap().to_string());
            } else if cont.contains(".c")
                || cont.contains(".asm")
                || cont.contains(".C")
                || cont.contains(".cla")
            {
                // source file
                self.srcfiles.push(cont.to_string());
            }
        }
    }

    /// Write compilation database to 'compile_commnads.json'
    pub fn write_to_json(&self) -> u8 {
        //
        if self.srcfiles.len() > 0 {
            //
            let json_file = "compile_commands.json";

            if let Ok(_file) = File::create(json_file) {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(json_file)
                    .expect("cannot open file");

                // Directory
                let mut directory: String = String::from("  \"directory\": \"");
                let tmp = path_transmit(self.directory.as_str());
                directory.push_str(&tmp);
                directory.push_str("\",\n");

                // compiler
                let mut compiler: String = String::from("      ");
                let tmp = path_transmit(self.compiler.as_str());
                compiler.push_str(&tmp);
                compiler.push_str(",\n");

                // include
                let mut include_str: String = String::new();
                for include_path in &self.include_path {
                    //
                    include_str.push_str("      \"-I\",\n");
                    include_str.push_str("      ");
                    let tmp = path_transmit(include_path.as_str());
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

                // Json file start [
                file.write_all("[\n".as_bytes()).expect("write failed");

                // Generates contents for each source file
                for srcfile in &self.srcfiles {
                    // --start-- { 
                    file.write_all(" {\n".as_bytes()).expect("write failed");

                    // "directory":
                    file.write_all(directory.as_bytes()).expect("write failed!");

                    // "file":
                    let mut line: String = String::from("  \"file\": ");
                    let tmp = path_transmit(srcfile.as_str());
                    line.push_str(&tmp);
                    line.push_str(",\n");
                    file.write_all(line.as_bytes()).expect("write failed!");

                    // argument start -- "arguments": [
                    file.write_all("  \"arguments\": [\n".as_bytes())
                        .expect("write failed");
                    // compiler
                    file.write_all(compiler.as_bytes()).expect("write failed!");

                    // include
                    file.write_all(include_str.as_bytes())
                        .expect("write failed!");

                    // defines
                    file.write_all(define_str.as_bytes())
                        .expect("write failed!");

                    // argument end -- ]
                    file.write_all("   ]\n".as_bytes()).expect("write failed");

                    // --end-- }
                    file.write_all(" },\n".as_bytes()).expect("write failed");
                }

                // json file end
                file.write_all("]".as_bytes()).expect("write failed");
                return 0;
            } else {
                return 1;
            }
        } else {
            return 1;
        }
    }
}

fn path_transmit(path_bef: &str) -> String {
    let mut path_aft: String = path_bef.replace("\\", "\\\\");
    path_aft = path_aft.replace("..", ".");
    path_aft = path_aft.replace("/", "\\\\");

    return path_aft;
}
