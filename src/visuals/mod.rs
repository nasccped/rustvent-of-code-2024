// declare files in this module (folder)
mod banners;

pub fn clear_terminal() {
    // importing Command
    use std::process::Command;

    // calling command (clear to unix like || cls to windows)
    let _ = Command::new("clear")
        .status()
        .or_else(|_| Command::new("cls").status());
}

pub fn get_escape(font_style: i32, foreground: i32, background: i32) -> String {
    let mut result: String = String::from("\x1b[0m");

    let font_escope: Vec<i32> = (0..8).filter(|x| *x != 2 || *x != 5 || *x != 6).collect();

    let foreground_escope: Vec<i32> = (30..38).collect();
    let background_escope: Vec<i32> = (40..48).collect();

    if font_style != 0 {
        result.push_str(&format!(
            "\x1b[{}m",
            match font_escope.contains(&font_style) {
                true => font_style,
                _ => 4,
            }
        ));
    }

    if foreground != 0 {
        result.push_str(&format!(
            "\x1b[{}m",
            match foreground_escope.contains(&foreground) {
                true => foreground,
                _ => 33,
            }
        ));
    }

    if background != 0 {
        result.push_str(&format!(
            "\x1b[{}m",
            match background_escope.contains(&background) {
                true => background,
                _ => 4,
            }
        ));
    }

    result
}

pub fn print_banner(banner_colors: (String, String)) {
    // local values that we'll use
    use banners::{print_merged, BELLOW_TAG, RUSTVENT_OF_CODE_TITLE};

    // calling print function passing these values
    print_merged(
        &RUSTVENT_OF_CODE_TITLE,            // title banner
        BELLOW_TAG,                         // bellow tag
        (banner_colors.0, banner_colors.1), // escapes
        (2, 1),                             // banner gaps
        (60, 0),                            // tag gaps
    );
}
