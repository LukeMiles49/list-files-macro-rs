# list_files_macro

[Crate](https://crates.io/crates/list_files_macro)

[Documentation](https://docs.rs/list_files_macro)

[Repository](https://github.com/LukeMiles49/list-files-macro-rs)

[Changelog](https://github.com/LukeMiles49/list-files-macro-rs/blob/master/CHANGELOG.md)

A simple proc macro to generate a const list of filenames and optionally apply a macro to each.

```rust
use std::fs::canonicalize;

use list_files_macro::list_files;

fn get_full_path(path: &str) -> String {
	canonicalize(path).unwrap().into_os_string().into_string().unwrap()
}

const FILENAMES: [&'static str; 3] = list_files!("../tests/files/*.rs");

assert_eq!(FILENAMES, [
	"tests/files/a.rs",
	"tests/files/b.rs",
	"tests/files/c.rs",
].map(get_full_path));

const CONTENTS: [&'static str; 3] = list_files!(include_str, "../tests/files/*.rs");

assert_eq!(CONTENTS[0], r#"
pub fn run() -> &'static str {
	"A"
}
"#);

macro_rules! run_file {
	($x:expr) => {
		{
			#[path = $x]
			mod file;
			file::run()
		}
	};
}

let results = list_files!(run_file, "../tests/files/*.rs");

assert_eq!(results, [
	"A",
	"B",
	"C",
]);
```

To use this, add it as a dependency to your Cargo.toml:
```toml
[dependencies]
list_files_macro = "^0.1.0"
```
