
use std::env;

fn main() {
    let args:Vec<String>  = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    // dbg!(args);  -- 删除测试代码

    println!("Searching for {}",query);
    println!("In file {}",file_path);

}
