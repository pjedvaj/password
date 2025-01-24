slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    use rand::Rng;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
                            
    let mut password_length: usize = 14;

    let mut password_range = rand::thread_rng();

    let password: String = (0..password_length)
        .map(|_| {
            let index = password_range.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();

    println!("{:?}", password);

    ui.run()
}