use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use human_bytes::human_bytes;
use log::debug;
use users::{get_group_by_gid, get_user_by_uid};

pub fn status_string<T: AsRef<Path>>(current_entry: T) -> String {
    let file_attributes = current_entry.as_ref().metadata();
    match file_attributes {
        Ok(attributes) => {
            let mut status_string = String::from("");
            let entry_permissions = attributes.permissions();
            let user_id = attributes.st_uid();
            let group_id = attributes.st_gid();
            let filesize_bytes = human_bytes(attributes.st_size() as f64);
            let _entry_last_modified = attributes.modified();

            let permission_string = unix_mode::to_string(entry_permissions.mode());

            status_string.push_str(&permission_string);
            status_string.push_str("  ");
            status_string.push_str(&get_username_from_id(user_id));
            status_string.push_str("  ");
            status_string.push_str(&get_groupname_from_id(group_id));
            status_string.push_str("  ");
            status_string.push_str(&filesize_bytes.to_string());

            status_string
        }
        Err(_) => {
            debug!("Unable to retreive file metadata.");
            "".to_string()
        }
    }
}

fn get_username_from_id(user_id: u32) -> String {
    let user = get_user_by_uid(user_id);
    match user {
        Some(user) => user.name().to_string_lossy().to_string(),
        None => user_id.to_string(),
    }
}

fn get_groupname_from_id(group_id: u32) -> String {
    let group = get_group_by_gid(group_id);
    match group {
        Some(group) => group.name().to_string_lossy().to_string(),
        None => group_id.to_string(),
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

        let status_bar_str = status_string(&test_filepath);
        let testfile_attributes = test_filepath.metadata().unwrap();

        let test_user = testfile_attributes.st_uid();
        let test_username = get_user_by_uid(test_user).unwrap().name().to_owned();
        let mut expected_string = String::from("-rw-r--r--  ");
        expected_string.push_str(test_username.to_str().unwrap());
        expected_string.push_str("  users  0 B");

        test_dir.close().unwrap();
        assert_eq!(expected_string, status_bar_str);
    }

    #[test]
    fn test_status_bar_tempdir() {
        let test_dir = tempdir().unwrap();
        let status_bar_str = status_string(test_dir.path().parent().unwrap());

        test_dir.close().unwrap();
        assert!(status_bar_str.contains("drwxrwxrwt  root  root  "));
    }
}
