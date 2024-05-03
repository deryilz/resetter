use std::{thread, time::Duration};

use clipboard::{ClipboardContext, ClipboardProvider};

pub fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

pub fn dist(coord1: (u32, u32), coord2: (u32, u32)) -> f64 {
    let x_diff = coord1.0.abs_diff(coord2.0);
    let y_diff = coord1.1.abs_diff(coord2.1);
    (x_diff as f64).hypot(y_diff as f64)
}

pub fn write_text(text: &str) {
    ClipboardContext::new()
        .unwrap()
        .set_contents(text.to_string())
        .expect("Couldn't set clipboard.");
    mki::Keyboard::LeftControl.press();
    mki::Keyboard::V.click();
    mki::Keyboard::LeftControl.release();
}
