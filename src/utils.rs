use std::{
    env,
    fs::{self, DirEntry, ReadDir},
    io::Error,
    path::{Path, PathBuf},
};

use log::{debug, error, log};

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
        debug!("Attempting to delete something that isn't a file or a dir.");
    }
}

pub fn copy_file<T: AsRef<Path>>(src_filepath: T, dest_filepath: T, move_contents: bool) {
    match fs::copy(src_filepath.as_ref(), dest_filepath.as_ref()) {
        Ok(_) => {
            if move_contents {
                delete_entry(src_filepath);
            }
        }
        Err(e) => {
            error!("Unable to copy file, Error: {:?}", e)
        }
    }
}

pub fn copy_directory<T: AsRef<Path>>(src_path: T, dest_dir: T, move_contents: bool) {
    if dest_dir.as_ref().exists() {
        // until I figure out a way to gracefully ask if user wants to overwrite
        // I'll just append _ to the end if there is already an identical file
        let mut appended_filename = src_path
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        appended_filename.push('_');
        let mut dest_dir = dest_dir.as_ref().to_owned();
        dest_dir.set_file_name(appended_filename);
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

        let mut license_filepath = src_dir;
        // license_filepath.push(&src_dir);
        license_filepath.push("LICENSE");

        let expected_file_contents = fs::read_to_string(&license_filepath).unwrap();

        copy_directory(license_filepath.as_path(), dest_dir.path(), false);

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

        copy_directory(src_dir.as_path(), dest_dir.path(), false);

        let mut result_filepath = PathBuf::new();
        result_filepath.push(&dest_dir);
        result_filepath.push("src");
        result_filepath.push("utils.rs");

        let result_file_contents = fs::read_to_string(result_filepath).unwrap();

        dest_dir.close().unwrap();

        assert_eq!(result_file_contents, expected_file_contents);
    }
}
