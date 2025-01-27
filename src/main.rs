#![windows_subsystem = "windows"]

use std::usize;

use slint::SharedString;
use slint::Weak;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle: Weak<AppWindow> = ui.as_weak();

    // Allowed characters
    const EMPTY: &[u8] = b"_";

    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

    const NUMBER: &[u8] = b"0123456789";

    const SPECIAL: &[u8] = b")(*&^%$#@!~";

    let mut charset:Vec<u8> = [EMPTY].concat();

    if ui.on_uppercase_password(move || {
        println!("uppercase toggled");
    }) == () {
        charset = [UPPERCASE].concat();
    }
    
    if ui.on_lowercase_password(move || {
        println!("Lowercase toggled");
    }) == () {
        charset = [UPPERCASE, LOWERCASE].concat();
    }
    
    if ui.on_number_password(move || {
        println!("Numbers toggled");
    }) == () {
        charset = [UPPERCASE, LOWERCASE, NUMBER].concat();
    }
    
    if ui.on_special_password(move || {
        println!("Special characters toggled");
    }) == () {
        charset = [UPPERCASE, LOWERCASE, NUMBER, SPECIAL].concat();
    }

    // Main
    ui.on_generate_password(move |long| {
        let ui: AppWindow = ui_handle.unwrap();

        // Password length
        ui.get_long();
        println!("Length: {}", ui.get_long());
        let mut str: String = long.to_string();
        let mut integer: usize = str.parse().expect("Not a valid number");
        let mut password_length = integer as usize;

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
    
        println!("Password: {:?}", password_string);

        // Convert String to SharedString for Slint UI
        let password = SharedString::from(password_string);
        
        // Set password variable in Slint UI
        ui.set_password(password);
    });

    ui.run()
}