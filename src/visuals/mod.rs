mod banners;

pub fn clear_terminal() {
    use std::process::Command;

    let _ = Command::new("clear")
        .status()
        .or_else(|_| Command::new("cls").status());
}

pub fn print_banner() {
    use banners::{print_merged, BANNER_ESCAPE, BELLOW_TAG, RUSTVENT_OF_CODE_TITLE, TAG_ESCAPE};

    print_merged(
        &RUSTVENT_OF_CODE_TITLE,
        BELLOW_TAG,
        (BANNER_ESCAPE, TAG_ESCAPE),
        (2, 1),
        (60, 0),
    );
}
