use clipboard::{ClipboardContext, ClipboardProvider};
use formatter::SQLFormatter;
use get_selected_text::get_selected_text;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

mod formatter;

fn main() {
    let formatter = SQLFormatter;

    let mut hkm = HotkeyManager::new();
    hkm.register(
        VKey::F,
        &[ModKey::Alt, ModKey::Shift],
        move || match get_selected_text() {
            Ok(selected_text) => {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(formatter.format_string(&selected_text).unwrap())
                    .unwrap();
            }
            Err(_) => {}
        },
    )
    .unwrap();

    hkm.event_loop();
}
