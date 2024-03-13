use std::env;  
use std::fs;  
use std::path::Path;  
  
fn main() {  
/*     eprintln!("starting");
    println!("starting!!!!!");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    eprintln!("out_dir:{}", out_dir.clone().into_string().unwrap());
    let mut dest_path = Path::new(&out_dir).join("log4rs.yaml");  
    
    let mut src_path = Path::new("./src/config/log4rs.yaml");  

    let mut dest_path_file = fs::File::open(dest_path.clone());
    eprintln!("test :  {}", dest_path_file.is_ok());

    let mut src_path_file = fs::File::open(src_path);
    println!("{}", src_path_file.is_ok());
      
    fs::copy(src_path, dest_path.clone()).expect("Failed to copy YAML file");   */
}