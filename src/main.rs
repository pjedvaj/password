#![windows_subsystem = "windows"]

use std::usize;

use slint::SharedString;
use slint::Weak;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle: Weak<AppWindow> = ui.as_weak();

    // Allowed characters
    const EMPTY: &[u8] = b"*";

    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

    const NUMBER: &[u8] = b"0123456789";

    const SPECIAL: &[u8] = b")(*&^%$#@!~";

    let mut charset:Vec<u8> = [EMPTY].concat();

    // Main
    ui.on_generate_password(move |length, up, low, num, spec| {
        let ui: AppWindow = ui_handle.unwrap();

        // Password length
        ui.get_length();
        let str: String = length.to_string();
        let integer: usize = str.parse().expect("Not a valid number");
        let password_length = integer as usize;

        // Password options
        ui.get_up();
        ui.get_low();
        ui.get_num();
        ui.get_spec();

        if up == false && low == false && num == false && spec == false {
            charset = [EMPTY].concat();
        }

        if up == true && low == false && num == false && spec == false {
            charset = [UPPERCASE].concat();
        }

        if up == true && low == true && num == false && spec == false {
            charset = [UPPERCASE, LOWERCASE].concat();
        }

        if up == true && low == true && num == true && spec == false {
            charset = [UPPERCASE, LOWERCASE, NUMBER].concat();
        }

        if up == true && low == true && num == true && spec == true {
            charset = [UPPERCASE, LOWERCASE, NUMBER, SPECIAL].concat();
        }

        if up == false && low == true && num == false && spec == false {
            charset = [LOWERCASE].concat();
        }

        if up == false && low == true && num == true && spec == false {
            charset = [LOWERCASE, NUMBER].concat();
        }

        if up == false && low == true && num == true && spec == true {
            charset = [LOWERCASE, NUMBER, SPECIAL].concat();
        }

        if up == false && low == false && num == true && spec == false {
            charset = [NUMBER].concat();
        }

        if up == false && low == false && num == false && spec == true {
            charset = [SPECIAL].concat();
        }

        if up == true && low == false && num == true && spec == false {
            charset = [UPPERCASE, NUMBER].concat();
        }

        if up == true && low == false && num == false && spec == true {
            charset = [UPPERCASE, SPECIAL].concat();
        }

        if up == false && low == true && num == false && spec == true {
            charset = [LOWERCASE, SPECIAL].concat();
        }

        if up == false && low == false && num == true && spec == true {
            charset = [NUMBER, SPECIAL].concat();
        }

        if up == true && low == true && num == false && spec == true {
            charset = [UPPERCASE, LOWERCASE, SPECIAL].concat();
        }

        // Random pasword generator
        use rand::Rng;
    
        // Generating password
        let mut password_range = rand::thread_rng();
    
        let password_string: String = (0..password_length)
            .map(|_| {
                let index = password_range.gen_range(0..charset.len());
                charset[index] as char
            })
            .collect();

        // Convert String to SharedString for Slint UI
        let password = SharedString::from(password_string);
        
        // Set password variable in Slint UI
        ui.set_password(password);
    });

    ui.run()
}