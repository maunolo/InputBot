use std::{thread::sleep, time::Duration};

use inputbot::KeybdKey::*;

/// This example demonstrates simulating mouse clicks.

fn main() {
    // Bind our Caps Lock key to a function that toggles autoclicking. Go AFK and bake some
    // cookies really fast without hurting your hands!
    CapsLockKey.bind(move || {
        while CapsLockKey.is_toggled() {
            WKey.press();
            sleep(Duration::from_millis(5000));
        }
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
