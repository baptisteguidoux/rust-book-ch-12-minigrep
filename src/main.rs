use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];    
    
    println!("searching {:?}", query);
    println!("in {}", filename);

    let contents = fs::read_to_string(filename)
	.expect("Could not read file");

    println!("file content:\n{}", contents);
}
