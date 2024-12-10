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

pub fn print_banner() {
    // local values that we'll use
    use banners::{print_merged, BANNER_ESCAPE, BELLOW_TAG, RUSTVENT_OF_CODE_TITLE, TAG_ESCAPE};

    // calling print function passing these values
    print_merged(
        &RUSTVENT_OF_CODE_TITLE,     // title banner
        BELLOW_TAG,                  // bellow tag
        (BANNER_ESCAPE, TAG_ESCAPE), // escapes
        (2, 1),                      // banner gaps
        (60, 0),                     // tag gaps
    );
}
