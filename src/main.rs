
use std::env;
use std::fs;

fn main() {
    let args:Vec<String>  = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    // dbg!(args);  -- 删除测试代码

    println!("Searching for {}",query);
    println!("In file {}",file_path);

    let contents = fs::read_to_string(file_path)
        .expect("should have been abel to read the file");

    println!("With text:\n{contents}");



}
