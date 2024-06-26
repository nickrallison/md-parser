mod parser;

pub mod md_parser {
	use std::fs;
	use std::path::PathBuf;
	use crate::parser::parse_md_file_wrapper;

	#[test]
	pub fn test_parse_a_star_search() {
		let path: PathBuf = "test/linked/a_star_search.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_ac_circuits() {
		let path: PathBuf = "test/linked/ac_circuits.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_access_clipboard_history() {
		let path: PathBuf = "test/linked/access_clipboard_history.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_address_resolution_protocol() {
		let path: PathBuf = "test/linked/address_resolution_protocol.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_algorithm_specifications() {
		let path: PathBuf = "test/linked/algorithm_specifications.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_analog_to_digital_converters() {
		let path: PathBuf = "test/linked/analog_to_digital_converters.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_arc_smart_pointer() {
		let path: PathBuf = "test/linked/arc_smart_pointer.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_atomic_types() {
		let path: PathBuf = "test/linked/atomic_types.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_adjacency_matrix() {
		let path: PathBuf = "test/linked/adjacency_matrix.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_amdahl_s_law() {
		let path: PathBuf = "test/linked/amdahl_s_law.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_antisymmetric_relation() {
		let path: PathBuf = "test/linked/antisymmetric_relation.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
	#[test]
	pub fn test_parse_bucket_sort() {
		let path: PathBuf = "test/linked/bucket_sort.md".into();
		let content = fs::read_to_string(&path).expect("Failed to read file");
		let parsed = parse_md_file_wrapper(content);
		println!("{:?}", parsed);
	}
}