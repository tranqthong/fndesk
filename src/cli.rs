use std::path::PathBuf;

use crate::utils;

pub fn parse_args(args: Vec<String>) -> PathBuf {
    let mut init_dir = utils::get_init_dirpath();

    if !args.is_empty() {
        if args.len() > 2 {
            println!("Too many arguments, starting with current directory.")
        } else if args.len() == 1 {
            return init_dir;
        } else {
            init_dir = PathBuf::new();
            init_dir.push(args[1].clone());

            if !init_dir.exists() {
                println!("Directory does not exist or non sufficient permission to open. Starting with current directory");
                init_dir = utils::get_init_dirpath()
            }
        }
    }

    init_dir
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_single_arg() {
        let args: Vec<String> = vec!["program".to_string()];
        let expected_path = fs::canonicalize(PathBuf::from(".")).unwrap();
        let result = parse_args(args);

        assert_eq!(result, expected_path)
    }

    #[test]
    fn test_valid_arg_path() {
        let tmp_dirpath = tempdir().unwrap();
        let args: Vec<String> = vec![
            "program".to_string(),
            tmp_dirpath.path().to_str().unwrap().to_string(),
        ];

        let result = parse_args(args);

        assert_eq!(result, tmp_dirpath.into_path());
    }

    #[test]
    fn test_too_many_args() {
        let args: Vec<String> = vec![
            "program".to_string(),
            "Hello".to_string(),
            "World".to_string(),
        ];
        let expected_path = fs::canonicalize(PathBuf::from(".")).unwrap();
        let result = parse_args(args);

        assert_eq!(result, expected_path);
    }

    #[test]
    fn test_invalid_dir() {
        let args: Vec<String> = vec!["program".to_string(), "fake_dir".to_string()];
        let expected_path = fs::canonicalize(PathBuf::from(".")).unwrap();

        let result = parse_args(args);

        assert_eq!(result, expected_path);
    }
}
