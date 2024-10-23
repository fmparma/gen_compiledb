use gen_compiledb::compiledb::LogPareser;
use std::env;

fn main() {
    println!("Hello, world!");

    let cwd: String = env::current_dir().unwrap().to_str().unwrap().to_string();
    let input_args: Vec<String> = env::args().collect();

    let buildlog_parser = LogPareser::new(&cwd, &input_args).unwrap();
    buildlog_parser
        .generate()
        .unwrap_or_else(|err| println!("Error : {}", err));
    //let path1 = env::current_exe().unwrap();

    //println!("current dir: {}", path.display());
    //println!("current exe dir: {}", path1.display());
}
