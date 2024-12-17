use std::{
    env,
    ffi::OsString,
    fs::{self, DirEntry},
    path::PathBuf,
};

pub fn get_parent_dir(selected_dir: &String) -> String {
    let path = PathBuf::from(selected_dir);

    path.parent()
        .unwrap()
        .as_os_str()
        .to_os_string()
        .into_string()
        .unwrap()
}

pub fn get_dir_items(selected_dir: &String, show_hidden: &bool) -> Vec<DirEntry> {
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

pub fn get_init_dirpath() -> OsString {
    let current_dir = env::current_dir().unwrap();
    current_dir.as_os_str().to_os_string()
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
        let result = get_parent_dir(&init_dir.into_string().unwrap());
        let expected = fs::canonicalize(PathBuf::from("..")).unwrap();

        assert_eq!(expected.to_str().unwrap().to_string(), result);
    }

    #[test]
    fn test_get_dir_items_no_hidden() {
        let init_dir = get_init_dirpath();
        let result = get_dir_items(&init_dir.into_string().unwrap(), &false);

        assert_eq!(5, result.len());
    }

    #[test]
    fn test_get_dir_items_all() {
        let init_dir = get_init_dirpath();
        let result = get_dir_items(&init_dir.into_string().unwrap(), &true);

        assert_eq!(8, result.len());
    }
}
