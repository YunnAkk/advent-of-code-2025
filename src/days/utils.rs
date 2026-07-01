use std::path::{Component, Path, PathBuf};

pub fn get_path_from_root(relative_path: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut path = root.to_path_buf();
    let relative_path = Path::new(relative_path);

    for component in relative_path.components() {
        match component {
            Component::Normal(c) => path.push(c),
            Component::ParentDir => {
                if path != root {
                    path.pop();
                }
            }
            Component::RootDir | Component::Prefix(_) => {}
            Component::CurDir => {}
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::MAIN_SEPARATOR;

    mod get_path_from_root {
        use super::*;

        #[test]
        fn happy_path() {
            let result = get_path_from_root("inputs/day01/input.txt");

            let expected = PathBuf::from(format!(
                "{}{sep}inputs{sep}day01{sep}input.txt",
                env!("CARGO_MANIFEST_DIR"),
                sep = MAIN_SEPARATOR
            ));

            assert_eq!(result, expected);
        }

        #[test]
        fn empty_path() {
            let result = get_path_from_root("");

            let expected = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

            assert_eq!(result, expected);
        }

        #[test]
        fn leading_separator_is_ignored() {
            let with_leading = get_path_from_root("/etc/passwd");

            let without = PathBuf::from(format!(
                "{}{sep}etc{sep}passwd",
                env!("CARGO_MANIFEST_DIR"),
                sep = MAIN_SEPARATOR
            ));

            assert_eq!(with_leading, without);
        }

        #[test]
        fn root_only_path() {
            let result = get_path_from_root("/");

            let expected = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

            assert_eq!(result, expected);
        }

        #[cfg(windows)]
        #[test]
        fn windows_drive_prefix_is_stripped() {
            let result = get_path_from_root(r"C:\Windows\System32");

            let expected = PathBuf::from(format!(
                "{}{sep}Windows{sep}System32",
                env!("CARGO_MANIFEST_DIR"),
                sep = MAIN_SEPARATOR
            ));

            assert_eq!(result, expected);
        }

        #[test]
        fn cur_dir_is_ignored() {
            let with_dot = get_path_from_root("./inputs/day01/input.txt");
            let without_dot = get_path_from_root("/inputs/day01/input.txt");

            assert_eq!(with_dot, without_dot);
        }

        #[test]
        fn pop_component() {
            let result = get_path_from_root("inputs/day01/..");

            let expected = PathBuf::from(format!(
                "{}{sep}inputs",
                env!("CARGO_MANIFEST_DIR"),
                sep = MAIN_SEPARATOR
            ));

            assert_eq!(result, expected);
        }

        #[test]
        fn pop_outside_parent() {
            let result = get_path_from_root("../../etc/passwd");

            let expected = PathBuf::from(format!(
                "{}{sep}etc{sep}passwd",
                env!("CARGO_MANIFEST_DIR"),
                sep = MAIN_SEPARATOR
            ));

            assert_eq!(result, expected);
        }

        #[test]
        fn trailing_separator_is_ignored() {
            let with_trailing = get_path_from_root("inputs/day01/");

            let without = PathBuf::from(format!(
                "{}{sep}inputs{sep}day01",
                env!("CARGO_MANIFEST_DIR"),
                sep = MAIN_SEPARATOR
            ));

            assert_eq!(with_trailing, without);
        }

        #[test]
        fn double_separators_are_normalized() {
            let result = get_path_from_root("inputs//day01/input.txt");

            let expected = PathBuf::from(format!(
                "{}{sep}inputs{sep}day01{sep}input.txt",
                env!("CARGO_MANIFEST_DIR"),
                sep = MAIN_SEPARATOR
            ));

            assert_eq!(result, expected);
        }
    }
}
