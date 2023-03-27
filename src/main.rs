// use std::io;
extern crate exitcode;
use std::fs::File;
use std::io::Write;
use std::path::Path;

struct Finder{
	pattern: String,
	file_path: String,
	output_file_name: String
}

impl Finder{
	fn search(self) {
		println!("Search in the file: {}", self.pattern);
		
		let content = std::fs::read_to_string(&self.file_path).expect("could not read file");
		
		let mut contents = String::new();
		for line in content.lines() {
		    if line.contains(&self.pattern) {
				contents += line;
				contents += "\n";
		        }
		}
		println!("{}", std::mem::size_of_val(&contents).to_string());
				
		let mut file = self.create_file();
		file.write_all(contents.as_bytes()).expect("Error encountered while creating file!");
	}
	
	fn create_file(mut self) -> File {
	    if Path::new(&self.output_file_name).exists() {
	        println!("File exists");
			self.output_file_name = "output_1.txt".to_string();
	    } else{
	        println!("File does not exist");
	    }  
				
	   let file = File::create(self.output_file_name)
		               .expect("Error encountered while creating file!");
	   file 
	}
}

fn main() -> std::io::Result<()> {
	let finder = Finder{pattern: std::env::args().nth(1).expect("no pattern given"), 
					    file_path: std::env::args().nth(2).expect("no path given"),
						output_file_name: String::from("output.txt")
						};
	
	finder.search();
		
	println!("Done");
	std::process::exit(exitcode::OK);
}
