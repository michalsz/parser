// use std::io;
extern crate exitcode;
use std::fs::File;
use std::io::Write;

fn main() {
	let pattern = std::env::args().nth(1).expect("no pattern given");
	let path = std::env::args().nth(2).expect("no path given");
	
	println!("search in the file: {}", pattern);
	
	let content = std::fs::read_to_string(path).expect("could not read file");
	
	
	let mut contents = String::new();
	for line in content.lines() {
	    if line.contains(&pattern) {
			contents += line;
			contents += "\n";
        }
	}

	let mut file = File::create("output.txt")
	               .expect("Error encountered while creating file!");
	
	file.write_all(contents.as_bytes()).expect("Error encountered while creating file!");
	
	println!("Done");
	std::process::exit(exitcode::OK);
}
