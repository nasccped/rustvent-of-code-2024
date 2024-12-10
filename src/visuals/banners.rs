// literals >>>
pub const RUSTVENT_OF_CODE_TITLE: [&str; 5] = [
    r#" ____            _                   _            __    ____          _      "#,
    r#"|  _ \ _   _ ___| |___   _____ _ __ | |_    ___  / _|  / ___|___   __| | ___ "#,
    r#"| |_) | | | / __| __\ \ / / _ \ '_ \| __|  / _ \| |_  | |   / _ \ / _` |/ _ \"#,
    r#"|  _ <| |_| \__ \ |_ \ V /  __/ | | | |_  | (_) |  _| | |__| (_) | (_| |  __/"#,
    r#"|_| \_\\__,_|___/\__| \_/ \___|_| |_|\__|  \___/|_|    \____\___/ \__,_|\___|"#,
];
pub const BELLOW_TAG: &str = "Nasccped - 2024";
pub const BANNER_ESCAPE: &str = "\x1b[1;37m";
pub const TAG_ESCAPE: &str = "\x1b[1;31m";

// function that print banner + tag and it's values
pub fn print_merged(
    banner: &[&str],
    str2: &str,
    each_escape: (&str, &str),
    gaps_banner: (i32, i32),
    gaps_str2: (i32, i32),
) {
    // printign y gaps for banner
    print!("{}", "\n".repeat(gaps_banner.1 as usize));

    // for each row in banner literal
    for row in banner.iter() {
        // print gap + escape + self row + reset escape/new line
        print!("{}", " ".repeat(gaps_banner.0 as usize));
        print!("{}{}", each_escape.0, row);
        println!("\x1b[0m");
    }

    // print y gaps for tag
    print!("{}", "\n".repeat(gaps_str2.1 as usize));

    // print gaps + escape + tag + reset escape
    println!(
        "{}{}{}\x1b[0m",
        " ".repeat(gaps_str2.0 as usize),
        each_escape.1,
        str2
    );
}
