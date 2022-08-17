use std::fs::canonicalize;

use list_files_macro::list_files;

fn get_full_path(path: &str) -> String {
	canonicalize(path).unwrap().into_os_string().into_string().unwrap()
}

#[test]
fn list_files() {
	const FILENAMES: [&'static str; 3] = list_files!("./files/*.rs");
	
	assert_eq!(FILENAMES, [
		"tests/files/a.rs",
		"tests/files/b.rs",
		"tests/files/c.rs",
	].map(get_full_path));
}

const FILEB: &'static str = r#"
pub fn run() -> &'static str {
	"A"
}
"#;

#[test]
fn load_files() {
	const CONTENTS: [&'static str; 3] = list_files!(include_str, "./files/*.rs");
	
	assert_eq!(CONTENTS[0], FILEB);
}

#[test]
fn run_files() {
	macro_rules! run_file {
		($x:expr) => {
			{
				#[path = $x]
				mod file;
				file::run()
			}
		};
	}
	
	let results = list_files!(run_file, "./files/*.rs");
	
	assert_eq!(results, [
		"A",
		"B",
		"C",
	]);
}