#![windows_subsystem = "windows"]

use slint::SharedString;
use slint::Weak;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle: Weak<AppWindow> = ui.as_weak();

    //Allowed characters
    const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

    const NUMBER: &[u8] = b"0123456789";

    const SPECIAL: &[u8] = b")(*&^%$#@!~";

    //Default use only uppercase letters
    let mut charset:Vec<u8> = [UPPERCASE].concat();

    //If checked also use lowercase letters
    ui.on_lowercase_password(move || {
        let mut charset:Vec<u8> = [UPPERCASE, LOWERCASE].concat();
        // println!("lowercase toggled");
    });

    //If checked also use numbers
    ui.on_number_password(move || {
        let mut charset:Vec<u8> = [UPPERCASE, LOWERCASE, NUMBER].concat();
        // println!("numbers toggled");
    });

    //If checked also use special characters
    ui.on_special_password(move || {
        let mut charset:Vec<u8> = [UPPERCASE, LOWERCASE, NUMBER, SPECIAL].concat();
        // println!("special characters toggled");
    });

    //Main
    ui.on_generate_password(move || {
        let ui: AppWindow = ui_handle.unwrap();

        //Random pasword generator
        use rand::Rng;

        // Password length - 16 characters
        let password_length: usize = 16;
    
        //Generating password
        let mut password_range = rand::thread_rng();
    
        let password_string: String = (0..password_length)
            .map(|_| {
                let index = password_range.gen_range(0..charset.len());
                charset[index] as char
            })
            .collect();
    
        //println!("{:?}", password_string);

        //Convert String to SharedString for Slint UI
        let password = SharedString::from(password_string);
        
        //Set password variable in Slint UI
        ui.set_password(password);
    });

    ui.run()
}