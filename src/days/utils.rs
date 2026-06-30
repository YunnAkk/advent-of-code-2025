use std::path::PathBuf;

pub fn construct_path(day: u8) -> PathBuf {
	let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	path.push("inputs");
	path.push(format!("day{:02}", day));
	path.push("input.txt");
	path
}

pub fn get_path_from_root(relative_path: &str) -> PathBuf {
	let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	path.push(relative_path);
	path
}