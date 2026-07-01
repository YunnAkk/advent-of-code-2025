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