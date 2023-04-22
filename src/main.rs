use std::env;
use std::process::Command;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: google <query>");
        return;
    }

    let query = &args[1..].join(" ");
    let encoded_query = utf8_percent_encode(query, NON_ALPHANUMERIC).to_string();
    let url = format!("https://www.google.com/search?q={}", encoded_query);

    #[cfg(target_os = "macos")]
    let _output = Command::new("open")
        .arg(url)
        .spawn()
        .expect("Failed to execute command");
    
    #[cfg(target_os = "windows")]
    let _output = Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(&url)
        .spawn()
        .expect("Failed to execute command");
}
