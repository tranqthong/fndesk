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



pub fn create_new_dir<T: AsRef<Path>>(root_path: T, new_dirpath: T) {

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
    } else {
        match fs::create_dir(dest_dir) {
            Ok(_) => {
                // TODO
            },
            Err(e) => {error!("Unable to create directory: {:?}", e)},
        }
    }
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
