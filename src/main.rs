use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use crate::parser::{MDFile, parse_md_file_wrapper};

mod parser;

fn main() {
	let folder_path: PathBuf = "test/linked".into();
	let filelist = fs::read_dir(folder_path).unwrap();
	for file in filelist {
		let file: DirEntry = file.unwrap();
		let path: PathBuf = file.path();
		println!("##############\nParsing: {}", path.display());
		let content: String = fs::read_to_string(&path).unwrap() + " \n";
		let parsed: MDFile = parse_md_file_wrapper(content);
		// println!("{:?}", parsed);
	}
}