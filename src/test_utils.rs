use crate::days::utils::get_path_from_root;
use std::path::PathBuf;

pub fn test_input_path(n: &str, filename: &str) -> PathBuf {
	get_path_from_root(&format!("test_inputs/day{n}/{filename}"))
}

#[macro_export]
macro_rules! define_day_in_path {
    ($day:literal) => {
        fn path_for(filename: &str) -> PathBuf {
            $crate::test_utils::test_input_path($day, filename)
        }
    };
}