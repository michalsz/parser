// use std::io;
extern crate exitcode;
use std::fs::File;
use std::io::Write;
use std::path::Path;

struct Finder{
	pattern: String,
	file_path: String
}

impl Finder{
	fn search(&self) {
		println!("Search in the file: {}", self.pattern);
		
		let content = std::fs::read_to_string(&self.file_path).expect("could not read file");
		
		let mut contents = String::new();
		for line in content.lines() {
		    if line.contains(&self.pattern) {
				contents += line;
				contents += "\n";
		        }
		}
				
		let mut file = self.create_file();
		file.write_all(contents.as_bytes()).expect("Error encountered while creating file!");
	}
	
	fn create_file(&self) -> File {
		let mut file_name = "output.txt";

	    if Path::new(file_name).exists() {
	        println!("File exists");
			file_name = "output_1.txt";
	    }
	    else{
	        println!("File does not exist");
	    }  
				
	   let file = File::create(file_name)
		               .expect("Error encountered while creating file!");
	   file 
	}
}

fn main() {
	let finder = Finder{pattern: std::env::args().nth(1).expect("no pattern given"), 
					    file_path: std::env::args().nth(2).expect("no path given")};
	
	finder.search();
		
	println!("Done");
	std::process::exit(exitcode::OK);
}
