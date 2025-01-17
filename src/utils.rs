use std::{
    env,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use log::debug;

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

pub fn get_init_dirpath() -> PathBuf {
    env::current_dir().expect("Current Directory does not exists or invalid permissions")
}

pub fn copy_dir_contents<T: AsRef<Path>>(source_dir: T, target_dir: T) {
    let source_entries = fs::read_dir(source_dir).unwrap();

    for entry in source_entries {
        match entry {
            Ok(entry) => {
                if entry.metadata().unwrap().is_file() {
                    let mut entry_target = PathBuf::new();
                    entry_target.push(target_dir.as_ref());
                    entry_target.push(entry.file_name());
                    let entry_copy = fs::copy(entry.path(), entry_target);
                    match entry_copy {
                        Ok(_) => debug!("Copy successful."),
                        Err(e) => debug!("Copy Failed: {e:?}"),
                    }
                } else if entry.metadata().unwrap().is_dir() {
                    // TODO
                    // can I handle this without recursion and keep it simple?
                } else {
                    debug!(
                        "Entry is neither file or directory. {:?}",
                        entry.file_name()
                    );
                }
            }
            Err(e) => debug!("Entry error: {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_init_dirpath() {
        let result = get_init_dirpath();
        let expected = fs::canonicalize(PathBuf::from(".")).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_parent_dir() {
        let init_dir = get_init_dirpath();
        let result = get_parent_dir(&init_dir);
        let expected = fs::canonicalize(PathBuf::from("..")).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_dir_items_no_hidden() {
        let init_dir = get_init_dirpath();
        let result = get_dir_items(&init_dir, &false);

        // there should only be six items found in the project root folder:
        // src/, target/, Cargo.lock, Cargo.toml, README.md, LICENSE
        assert_eq!(6, result.len());
    }

    #[test]
    fn test_get_dir_items_all() {
        let init_dir = get_init_dirpath();
        let result = get_dir_items(&init_dir, &true);

        // like the above, but with 8 counting 3 hidden dir/files:
        // .git/, .gitignore, .vscode/
        assert_eq!(9, result.len());
    }
}
