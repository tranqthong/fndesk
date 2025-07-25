use std::{
    env,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

pub fn get_parent_dir<T: AsRef<Path>>(selected_dir: T) -> PathBuf {
    selected_dir
        .as_ref()
        .parent()
        .unwrap_or(selected_dir.as_ref())
        .to_path_buf()
}

pub fn get_dir_items<T: AsRef<Path>>(selected_dir: T, show_hidden: &bool) -> Vec<DirEntry> {
    let mut item_paths: Vec<_> = fs::read_dir(selected_dir)
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    if !show_hidden {
        item_paths.retain(|x| !x.file_name().into_string().unwrap().starts_with("."));
    }
    item_paths.sort_by_key(|x| x.path());

    item_paths
}

pub fn get_current_dirpath() -> PathBuf {
    env::current_dir().expect("Current Directory does not exists or invalid permissions")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_dirpath() {
        let result = get_current_dirpath();
        let expected = fs::canonicalize(PathBuf::from(".")).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_parent_dir() {
        let init_dir = get_current_dirpath();
        let result = get_parent_dir(&init_dir);
        let expected = fs::canonicalize(PathBuf::from("..")).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_dir_items_no_hidden() {
        let init_dir = get_current_dirpath();
        let result = get_dir_items(&init_dir, &false);

        // there are currently six items found in the project root folder:
        // src/, target/, Cargo.lock, Cargo.toml, README.md, LICENSE
        assert_eq!(6, result.len());
    }

    #[test]
    fn test_get_dir_items_all() {
        let init_dir = get_current_dirpath();
        let result = get_dir_items(&init_dir, &true);

        // like the above, but with 8 counting 3 hidden dir/files:
        // .git/, .gitignore, .vscode/
        assert_eq!(9, result.len());
    }
}
