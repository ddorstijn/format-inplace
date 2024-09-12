use formatter::{IndentationType, SQLFormatter};
use get_selected_text::get_selected_text;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

mod formatter;

fn main() {
    let mut hkm = HotkeyManager::new();

    hkm.register(
        VKey::F,
        &[ModKey::Alt, ModKey::Shift],
        || match get_selected_text() {
            Ok(selected_text) => {
                println!(
                    "{:?}",
                    SQLFormatter::new(IndentationType::Tabbed).format_string(&selected_text)
                );
            }
            Err(_) => {}
        },
    )
    .unwrap();

    hkm.event_loop();
}
