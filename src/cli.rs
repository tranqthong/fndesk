use std::path::PathBuf;

use crate::utils;

#[allow(deprecated)] // TODO remove after std::env::home has been undeprecated
pub fn parse_args(args: Vec<String>) -> PathBuf {
    let mut init_dir = utils::get_current_dirpath();

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
                // if for some reason we can't open the user specified dir
                // then we default to either the home_dir based on the user's env
                // otherwise we just start with the current directory
                // std::env::home_dir will be undeprecated in the next rust release
                init_dir = match std::env::home_dir() {
                    Some(x) => x,
                    None => utils::get_current_dirpath(),
                };
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

        assert_eq!(result, tmp_dirpath.as_ref().to_path_buf());
        tmp_dirpath.close().unwrap();
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
        let expected_path = std::env::home_dir().unwrap();

        let result = parse_args(args);

        assert_eq!(result, expected_path);
    }
}
