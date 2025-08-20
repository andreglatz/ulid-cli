use arboard::Clipboard;
use clap::Parser;
use colored::Colorize;
use std::thread;
use std::time::Duration;
use ulid::Ulid;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    clipboard: bool,
}

fn main() {
    let args = Args::parse();

    let ulid = Ulid::new();
    let s = ulid.to_string();
    println!("{}", s);

    if args.clipboard {
        copy_to_clipboard(&s);
    }
}

fn copy_to_clipboard(text: &str) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();

    thread::sleep(Duration::from_millis(100));

    println!("{}", "Copied to clipboard!".green());
}
