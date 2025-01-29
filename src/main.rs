//#![windows_subsystem = "windows"]

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

    // Main
    ui.on_generate_password(move |length, option| {
        let ui: AppWindow = ui_handle.unwrap();

        // Password length
        ui.get_length();
        println!("Length: {}", ui.get_length());
        let str: String = length.to_string();
        let integer: usize = str.parse().expect("Not a valid number");
        let password_length = integer as usize;

        // Password options
        ui.get_option();
        let option = ui.get_option();
        println!("Option: {}", option);

        if option == 1 {
            charset = [UPPERCASE].concat();
        }

        else if option == 3 {
            charset = [UPPERCASE, LOWERCASE].concat();
        }

        else if option == 5 {
            charset = [UPPERCASE, LOWERCASE, NUMBER].concat();
        }

        else if option == 7 {
            charset = [UPPERCASE, LOWERCASE, NUMBER, SPECIAL].concat();
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
    
        println!("Password: {:?}", password_string);

        // Convert String to SharedString for Slint UI
        let password = SharedString::from(password_string);
        
        // Set password variable in Slint UI
        ui.set_password(password);
    });

    ui.run()
}