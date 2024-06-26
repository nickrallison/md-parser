use std::fs;
use std::path::PathBuf;
use crate::parser::parse_md_file_wrapper;

mod parser;

fn main() {
	let folder_path: PathBuf = "test/linked".into();
	let filelist = fs::read_dir(folder_path).unwrap();
	for file in filelist {
		let file = file.unwrap();
		let path = file.path();
		let content = fs::read_to_string(&path).unwrap();
		println!("##############\nParsing: {}", path.display());
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
}