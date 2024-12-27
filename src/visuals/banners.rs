const RUSTVENT_OF_CODE_TITLE: [&str; 5] = [
    r#" ____            _                   _            __    ____          _      "#,
    r#"|  _ \ _   _ ___| |___   _____ _ __ | |_    ___  / _|  / ___|___   __| | ___ "#,
    r#"| |_) | | | / __| __\ \ / / _ \ '_ \| __|  / _ \| |_  | |   / _ \ / _` |/ _ \"#,
    r#"|  _ <| |_| \__ \ |_ \ V /  __/ | | | |_  | (_) |  _| | |__| (_) | (_| |  __/"#,
    r#"|_| \_\\__,_|___/\__| \_/ \___|_| |_|\__|  \___/|_|    \____\___/ \__,_|\___|"#,
];
const BELLOW_TAG: &str = "Nasccped - 2024";

pub fn print_banner() {
    println!("\x1b[1;33m");
    println!(
        "{}",
        RUSTVENT_OF_CODE_TITLE
            .iter()
            .map(|x| format!("  {}", x))
            .collect::<Vec<_>>()
            .join("\n")
    );
    println!("  {}\x1b[1;31m{}\x1b[0m", " ".repeat(62), BELLOW_TAG);
    println!("  \x1b[1;37m{}\x1b[0m", "=".repeat(78));
}
