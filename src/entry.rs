use std::{
    fs, io,
    path::{Path, PathBuf},
};

use log::{debug, error};

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

pub fn append_duplicates<T: AsRef<Path>>(src_entry: T, dest_entry: T) -> PathBuf {
    let src_filename = src_entry.as_ref().file_name().unwrap();

    let mut dest_path = PathBuf::new();
    dest_path.push(dest_entry);
    dest_path.push(src_filename);

    if dest_path.exists() {
        let mut new_filename = src_filename.to_owned().into_string().unwrap();
        new_filename.push('_');
        dest_path.pop();
        dest_path.push(new_filename);
    }
    dest_path
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

pub fn copy_dir<T: AsRef<Path>>(
    src_dirpath: T,
    dest_dirpath: T,
    move_contents: bool,
) -> io::Result<()> {
    fs::create_dir_all(&dest_dirpath)?;
    for entry in fs::read_dir(src_dirpath)? {
        let entry = entry?;
        let entry_type = entry.file_type()?;
        if entry_type.is_dir() {
            copy_dir(
                entry.path(),
                dest_dirpath.as_ref().join(entry.file_name()),
                move_contents,
            )?;
        } else {
            let appended_destpath =
                append_duplicates(entry.path().as_path(), dest_dirpath.as_ref());
            copy_file(entry.path(), appended_destpath, move_contents);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path;
    use std::path::PathBuf;
    use tempfile::tempdir;

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
        let src_dir = path::get_current_dirpath();
        let dest_dir = tempdir().unwrap();

        let mut license_filepath = src_dir;
        license_filepath.push("LICENSE");

        let expected_file_contents = fs::read_to_string(&license_filepath).unwrap();
        let dest_dirpath = append_duplicates(license_filepath.as_path(), dest_dir.path());

        copy_file(license_filepath.as_path(), &dest_dirpath, false);

        let result_file_contents = fs::read_to_string(dest_dir.path().join("LICENSE")).unwrap();

        dest_dir.close().unwrap();

        assert_eq!(result_file_contents, expected_file_contents);
    }

    #[test]
    fn test_copy_subdir_file() {
        let project_dir = path::get_current_dirpath();
        let dest_dir = tempdir().unwrap();

        let mut main_rs_filepath = PathBuf::new();
        main_rs_filepath.push(&project_dir);
        main_rs_filepath.push("src");
        main_rs_filepath.push("main.rs");

        let expected_file_contents = fs::read_to_string(&main_rs_filepath).unwrap();

        let copy_result = copy_dir(project_dir.as_path(), dest_dir.path(), false);

        let mut result_filepath = PathBuf::new();
        result_filepath.push(&dest_dir);
        result_filepath.push("src");
        result_filepath.push("main.rs");

        let result_file_contents = fs::read_to_string(result_filepath).unwrap();

        dest_dir.close().unwrap();

        assert!(copy_result.is_ok());
        assert_eq!(result_file_contents, expected_file_contents);
    }

    #[test]
    fn test_copy_duplicate_file() {
        let src_dir = path::get_current_dirpath();
        let dest_dir = tempdir().unwrap();

        let mut license_filepath = src_dir;
        license_filepath.push("LICENSE");

        let expected_file_contents = fs::read_to_string(&license_filepath).unwrap();

        let dest_dirpath = append_duplicates(license_filepath.as_path(), dest_dir.path());
        copy_file(license_filepath.as_path(), &dest_dirpath, false);

        let dest_dirpath = append_duplicates(license_filepath.as_path(), dest_dir.path());
        copy_file(license_filepath.as_path(), &dest_dirpath, false);

        let result_file_contents = fs::read_to_string(dest_dir.path().join("LICENSE_")).unwrap();

        dest_dir.close().unwrap();

        assert_eq!(result_file_contents, expected_file_contents);
    }

    #[test]
    fn test_copy_subdir_duplicates() {
        let project_dir = path::get_current_dirpath();
        let dest_dir = tempdir().unwrap();

        let mut main_rs_filepath = PathBuf::new();
        main_rs_filepath.push(&project_dir);
        main_rs_filepath.push("src");
        main_rs_filepath.push("main.rs");

        let expected_file_contents = fs::read_to_string(&main_rs_filepath).unwrap();

        let first_copy_result = copy_dir(project_dir.as_path(), dest_dir.path(), false);

        let second_copy_result = copy_dir(project_dir.as_path(), dest_dir.path(), false);

        let mut result_filepath = PathBuf::new();
        result_filepath.push(&dest_dir);
        result_filepath.push("src");
        result_filepath.push("main.rs_");

        let result_file_contents = fs::read_to_string(result_filepath).unwrap();

        dest_dir.close().unwrap();

        assert!(first_copy_result.is_ok());
        assert!(second_copy_result.is_ok());
        assert_eq!(result_file_contents, expected_file_contents);
    }

    #[test]
    fn test_move_file() {
        let src_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();

        let test_filepath = src_dir.path().join("test_file.txt");
        let _test_file = fs::File::create(&test_filepath).unwrap();

        let dest_filepath = dest_dir.path();

        let dest_filepath = append_duplicates(test_filepath.as_path(), dest_filepath);

        copy_file(test_filepath.as_path(), dest_filepath.as_path(), true);

        assert!(!test_filepath.exists());
        assert!(dest_filepath.exists());

        src_dir.close().unwrap();
        dest_dir.close().unwrap();
    }
}
