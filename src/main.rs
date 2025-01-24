#![windows_subsystem = "windows"]

use slint::SharedString;
use slint::Weak;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle: Weak<AppWindow> = ui.as_weak();

    ui.on_generate_password(move || {
        let ui: AppWindow = ui_handle.unwrap();

        //Random pasword generator
        use rand::Rng;

        //All allowed characters
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~";

        // Password length - 16 characters
        let password_length: usize = 16;
    
        //Generating password
        let mut password_range = rand::thread_rng();
    
        let password_string: String = (0..password_length)
            .map(|_| {
                let index = password_range.gen_range(0..CHARSET.len());
                CHARSET[index] as char
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