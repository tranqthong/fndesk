use std::ffi::OsStr;

use ratatui::style::{palette::tailwind::SLATE, Color};

const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;

pub fn get_file_name(path: &OsStr) -> String {
    path.to_string_lossy().into_owned()
}

pub const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_get_file_name() {
        let path = OsStr::new("path/to/test_file.txt");
        assert_eq!(get_file_name(path), "test_file.txt".to_string());
    }

    #[test]
    fn test_get_file_name_blank() {
        let path = OsStr::new("");
        assert_eq!(get_file_name(path), "".to_string());
    }
}
