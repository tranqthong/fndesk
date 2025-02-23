use std::{
    env,
    fs::{self, DirEntry},
    io::Error,
    path::{Path, PathBuf},
};

use log::{debug, error};

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

pub fn delete_entry<T: AsRef<Path>>(selected_entry: T) {
    if selected_entry.as_ref().is_file() {
        let file_delete = fs::remove_file(selected_entry);
        match file_delete {
            Ok(_) => {}
            Err(e) => debug!("Unable to delete file: {e:?}"),
        }
    } else if selected_entry.as_ref().is_dir() {
        // WARNING, this will delete the directory and all of its contents, including subdirectories
        let dir_delete = fs::remove_dir_all(selected_entry);
        match dir_delete {
            Ok(_) => {}
            Err(e) => debug!("Unable to delete dir, check permissions: {e:?}"),
        }
    } else {
        debug!("Attempting to delete something that isn't a file or a dir???");
    }
}

pub fn copy_file<T: AsRef<Path>>(src_entry: DirEntry, dest_dir: T) -> Result<u64, Error> {
    let mut entry_dest_path = PathBuf::new();
    entry_dest_path.push(dest_dir.as_ref());
    entry_dest_path.push(src_entry.file_name());
    if entry_dest_path.exists() {
        // until I figure out a way to gracefully ask if user wants to overwrite
        // I'll just append _ to the end if there is already an identical file
        let mut appended_dest_filename = src_entry.file_name().into_string().unwrap();
        appended_dest_filename.push('_');
        entry_dest_path.set_file_name(&appended_dest_filename);
    }

    fs::copy(src_entry.path(), entry_dest_path)
}

pub fn copy_dir_contents<T: AsRef<Path>>(source_dir: T, dest_dir: T) {
    let source_entries = fs::read_dir(source_dir).unwrap();

    for entry in source_entries {
        match entry {
            Ok(entry) => {
                if entry.metadata().unwrap().is_file() {
                    match copy_file(entry, &dest_dir) {
                        Ok(_) => (),
                        // if the copy fails, we will move on to the next file
                        Err(e) => error!("{e:?}"),
                    };
                } else if entry.metadata().unwrap().is_dir() {
                    let mut dest_subdir = PathBuf::new();
                    dest_subdir.push(&dest_dir);
                    dest_subdir.push(entry.file_name());
                    if !dest_subdir.exists() {
                        let create_subdir = fs::create_dir(&dest_subdir);
                        match create_subdir {
                            Ok(_) => (),
                            Err(e) => {
                                debug!("Unable to create directory, skipping... {e:?}");
                                continue;
                            }
                        }
                    }
                    copy_dir_contents(entry.path(), dest_subdir);
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
    use tempfile::tempdir;

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

    #[test]
    fn test_delete_file() {
        let test_dir = tempdir().unwrap();
        let test_filepath = test_dir.path().join("test_file.txt");
        let _test_file = fs::File::create(&test_filepath).unwrap();

        delete_entry(&test_filepath);

        assert!(!test_filepath.exists());
        test_dir.close().unwrap();
    }

    #[test]
    fn test_delete_dir() {
        let test_dir = tempdir().unwrap();
        let test_dirpath = test_dir.path();

        delete_entry(&test_dirpath);

        assert!(!test_dirpath.exists());
    }

    #[test]
    fn test_copy_single_file() {
        let src_dir = get_current_dirpath();
        let dest_dir = tempdir().unwrap();

        let mut license_filepath = PathBuf::new();
        license_filepath.push(&src_dir);
        license_filepath.push("LICENSE");

        let expected_file_contents = fs::read_to_string(license_filepath).unwrap();

        copy_dir_contents(src_dir.as_path(), dest_dir.path());

        let result_file_contents = fs::read_to_string(dest_dir.path().join("LICENSE")).unwrap();

        dest_dir.close().unwrap();

        assert_eq!(result_file_contents, expected_file_contents);
    }

    #[test]
    fn test_copy_subdir_file() {
        let src_dir = get_current_dirpath();
        let dest_dir = tempdir().unwrap();

        let mut self_filepath = PathBuf::new();
        self_filepath.push(&src_dir);
        self_filepath.push("src");
        self_filepath.push("utils.rs");

        let expected_file_contents = fs::read_to_string(self_filepath).unwrap();

        copy_dir_contents(src_dir.as_path(), dest_dir.path());

        let mut result_filepath = PathBuf::new();
        result_filepath.push(&dest_dir);
        result_filepath.push("src");
        result_filepath.push("utils.rs");

        let result_file_contents = fs::read_to_string(result_filepath).unwrap();

        dest_dir.close().unwrap();

        assert_eq!(result_file_contents, expected_file_contents);
    }
}
