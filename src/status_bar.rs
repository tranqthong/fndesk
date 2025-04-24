use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use users::{get_group_by_gid, get_user_by_uid};

pub fn status_bar<T: AsRef<Path>>(current_entry: T) -> String {
    let file_attributes = current_entry.as_ref().metadata();
    match file_attributes {
        Ok(attributes) => {
            let mut status_string = String::from("");
            let entry_permissions = attributes.permissions();
            let user_id = attributes.st_uid();
            let group_id = attributes.st_gid();
            let filesize_bytes = attributes.st_size();
            let entry_last_modified = attributes.modified();

            let user = get_user_by_uid(user_id).unwrap();
            let username_str = user.name();
            let group_str = get_group_by_gid(group_id).unwrap().name().to_owned();
            let permission_string = unix_mode::to_string(entry_permissions.mode());

            status_string.push_str(&permission_string);
            status_string.push_str("  ");
            status_string.push_str(username_str.to_str().unwrap());
            status_string.push_str("  ");
            status_string.push_str(group_str.to_str().unwrap());
            status_string.push_str("  ");
            status_string.push_str(&filesize_bytes.to_string());
            // status_string.push_str("{}")

            status_string
        }
        Err(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use tempfile::tempdir;

    #[test]
    fn test_status_bar_tempfile() {
        let test_dir = tempdir().unwrap();
        let test_filepath = test_dir.path().join("test_file.txt");
        let _test_file = fs::File::create(&test_filepath).unwrap();

        let status_bar_str = status_bar(&test_filepath);
        let testfile_attributes = test_filepath.metadata().unwrap();

        let test_user = testfile_attributes.st_uid();
        let test_username = get_user_by_uid(test_user).unwrap().name().to_owned();
        let mut expected_string = String::from("-rw-r--r--  ");
        expected_string.push_str(test_username.to_str().unwrap());
        expected_string.push_str("  users  0");

        test_dir.close().unwrap();
        assert_eq!(expected_string, status_bar_str);
    }

    #[test]
    fn test_status_bar_tempdir() {
        let test_dir = tempdir().unwrap();
        let status_bar_str = status_bar(test_dir.path().parent().unwrap());

        assert_eq!("drwxrwxrwt  root  root  740", status_bar_str);
    }
}
