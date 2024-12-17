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
        item_paths.retain(|x| !x.file_name().into_string().unwrap().contains("."));
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
        let expected = env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        assert_eq!(expected, result.into_string().unwrap());
    }

    #[test]
    fn test_get_parent_dir() {}

    #[test]
    fn test_get_dir_items() {}
}
