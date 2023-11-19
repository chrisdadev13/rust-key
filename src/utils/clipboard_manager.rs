use clipboard::{ClipboardContext, ClipboardProvider};

pub fn clipboard_manager(text_to_copy: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    if let Err(err) = ctx.set_contents(text_to_copy.to_owned()) {
        eprintln!("Error copying to clipboard: {}", err);
    } else {
        println!("Password copied to clipboard: {}", text_to_copy);
    }
}
