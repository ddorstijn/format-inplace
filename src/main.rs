use std::thread::sleep;

use arboard::{Clipboard, ImageData};
use enigo::{Enigo, Keyboard, Settings};
use formatter::format_string;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

mod formatter;

#[derive(Debug)]
enum ClipboardContent {
    Text(String),
    Image(ImageData<'static>),
}

fn format_query() -> Result<(), Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Unpress current keys
    enigo.key(enigo::Key::Shift, enigo::Direction::Release)?;
    enigo.key(enigo::Key::Alt, enigo::Direction::Release)?;

    enigo.key(enigo::Key::F, enigo::Direction::Release)?;

    let mut clipboard = Clipboard::new().unwrap();

    let clipboard_content = match clipboard.get_text() {
        Ok(text) => Some(ClipboardContent::Text(text)),
        Err(_) => match clipboard.get_image() {
            Ok(image) => Some(ClipboardContent::Image(image)),
            Err(_) => None,
        },
    };

    // Press CTRL + C
    enigo.key(enigo::Key::Control, enigo::Direction::Press)?;
    enigo.key(enigo::Key::C, enigo::Direction::Press)?;
    enigo.key(enigo::Key::C, enigo::Direction::Release)?;
    enigo.key(enigo::Key::Control, enigo::Direction::Release)?;

    sleep(std::time::Duration::from_millis(50));

    let selected_text = match clipboard.get_text() {
        Ok(t) => t,
        Err(_) => {
            // One more retry
            println!("No clipboard entry found. Retrying...");
            sleep(std::time::Duration::from_millis(500));
            clipboard.get_text()?
        }
    };

    if let Ok(formatted_text) = format_string(&selected_text) {
        println!("--- Formatted query: \n {}", formatted_text);
        clipboard.set_text(formatted_text)?;

        sleep(std::time::Duration::from_millis(500));

        // Paste formatted text
        enigo.key(enigo::Key::Control, enigo::Direction::Press)?;
        enigo.key(enigo::Key::V, enigo::Direction::Press)?;

        sleep(std::time::Duration::from_millis(50));

        enigo.key(enigo::Key::V, enigo::Direction::Release)?;
        enigo.key(enigo::Key::Control, enigo::Direction::Release)?;

        sleep(std::time::Duration::from_millis(500));

        if let Some(clipboard_content) = clipboard_content {
            match clipboard_content {
                ClipboardContent::Text(text) => clipboard.set_text(text)?,
                ClipboardContent::Image(image) => clipboard.set_image(image)?,
            };
        };
    };

    Ok(())
}

fn main() {
    let mut hkm = HotkeyManager::new();
    hkm.register(VKey::F, &[ModKey::Alt, ModKey::Shift], move || {
        if let Err(err) = format_query() {
            println!("{:?}", err);
        }
    })
    .unwrap();

    hkm.event_loop();
}
