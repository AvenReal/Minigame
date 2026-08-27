use std::{thread::sleep, time::Duration};

use crate::commands;
use terminal_size::{Height, Width, terminal_size};

pub fn print_on_screen(y: usize, x: usize, message: &str) {
    println!("\x1B[{};{}H{}", y, x, message);
}

/// .
pub fn print_alert(message: &str) {
    
    let n = message.len();
    print_on_screen(5, n / 2, message);
    sleep(Duration::new(3, 0));
    print_on_screen(5, n / 2, &" ".repeat(n));
}

pub fn clear(){
    let (h, w) = match get_screen_size(){
        Ok((h, w)) => (h, w),
        Err(_) => return,
    };

    for i in 1..=h {
        println!("\x1B[{i};1H{}", &" ".repeat(w as usize));
    }
}

/// Retur the screen size as (Height : u16, Width : u16).
///
/// # Errors
///
/// This function will return an error if the screen size is unable to be read.
pub fn get_screen_size() -> Result<(u16, u16), commands::Command> {
    return if let Some((Width(w), Height(h))) = terminal_size() {
        Ok((h, w))
    } else {
        Err(commands::Command::Invalid(String::from("Unable to get the terminal size !")))
    }
}
