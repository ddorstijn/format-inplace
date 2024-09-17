use std::path::PathBuf;

use clap::Parser;
use formatter::{IndentationType, SQLFormatter};
use get_selected_text::get_selected_text;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

mod formatter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "tabbed")]
    indent_type: IndentationType,

    #[arg(short, long, default_value_t = false)]
    deamon: bool,

    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let formatter = SQLFormatter::new(args.indent_type);

    if args.deamon && args.file.is_some() {
        println!("Error: Cannot specify both --deamon and --file at the same time.");
        std::process::exit(1);
    }

    if !args.deamon && args.file.is_none() {
        println!("Error: Must specify either --deamon or --file.");
        std::process::exit(1);
    }

    if let Some(file) = args.file {
        formatter.format_file(file).unwrap();
    }

    if args.deamon {
        let mut hkm = HotkeyManager::new();
        hkm.register(
            VKey::F,
            &[ModKey::Alt, ModKey::Shift],
            move || match get_selected_text() {
                Ok(selected_text) => {
                    println!("{:?}", formatter.format_string(&selected_text));
                }
                Err(_) => {}
            },
        )
        .unwrap();

        hkm.event_loop();
    }
}
