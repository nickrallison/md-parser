mod parser;

pub mod md_parser {
	use std::path::PathBuf;
	use crate::parser::parse_md_file_wrapper;

	#[test]
	pub fn parse_shortest_path_tree() {
		let path: PathBuf = "test/linked/shortest_path_tree.md".into();
		let mut parsed = parse_md_file_wrapper::MdParser::new(path);
		println!("{:?}", parsed);
	}
}